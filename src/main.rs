//! # Load test an OCI compliant registry
use std::env;

use oci_distribution::{client::ClientProtocol, secrets::RegistryAuth};
use tracing::{error, info};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use clap::{arg, Command};
fn cli() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(arg!(-v --verbose "Print verbose output").default_value("false"))
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("push-images")
                .about("Pushes a generated OCI to a distribution server")
                .arg(
                    arg!(--reg_url <REGISTRY_URL> "The distribution server")
                        .default_value("http://localhost:6000"),
                )
                .arg(arg!(--reg_user <REGISTRY_USER> "The user to authenticate against the distribution server"))
                .arg(arg!(--reg_pass <REGISTRY_PASSWORD> "The password to authenticate against the distribution server"))
                .arg(arg!(-c --count <COUNT> "The amount of images to push").default_value("1"))
                .arg_required_else_help(false),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    let verbose: &bool = matches.get_one("verbose").unwrap();

    let mut filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    if *verbose {
        filter = EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into());
    }

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .unwrap();

    match matches.subcommand() {
        Some(("push-images", matches)) => {
            let registry_url = matches.get_one::<String>("reg_url").unwrap();
            let count = matches.get_one::<String>("count").unwrap();
            let count = count.parse::<usize>().unwrap();

            info!(count = count, registry_url = registry_url, "Pushing images");
            let reg_url = url::Url::parse(registry_url).unwrap();
            let reg_host = reg_url.host_str().unwrap();
            let reg_port = reg_url.port();
            let reg_protocol = reg_url.scheme();

            let mut reg = reg_host.to_string();

            if let Some(port) = reg_port {
                reg.push(':');
                reg.push_str(&port.to_string());
            }

            let protocol = match reg_protocol {
                "http" => ClientProtocol::Http,
                "https" => ClientProtocol::Https,
                _ => panic!("Unknown protocol"),
            };

            let mut auth = RegistryAuth::Anonymous;
            let user: Option<String> = matches.get_one::<String>("reg_user").map(Clone::clone);
            let password: Option<String> = matches.get_one::<String>("reg_pass").map(Clone::clone);
            if let Some(user) = user {
                if let Some(password) = password {
                    auth = RegistryAuth::Basic(user, password);
                }
            }
            info!(
                image_count = count,
                registry_url = registry_url,
                "Starting load test"
            );
            let results = oci_distribution_tester::load_test(count, reg, auth, protocol).await;

            let total = results.len();
            let success = results
                .into_iter()
                .map(|r| r.map_err(|e| error!("{e}")))
                .filter(Result::is_ok)
                .count();

            info!(
                "Total: {total}, Success: {success}",
                total = total,
                success = success
            );
        }
        _ => unreachable!(),
    }
}
