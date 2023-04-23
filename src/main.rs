//! # Load test an OCI compliant registry
use anyhow::{anyhow, bail, Context, Result};
use clap::{arg, ArgMatches, Command};
use oci_distribution::{client::ClientProtocol, secrets::RegistryAuth, Reference};
use tracing::{error, info};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

fn cli() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(arg!(-v --verbose "Print verbose output").default_value("false").global(true))
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("push-images")
                .about("Pushes a generated OCI to a distribution server")
                .arg(
                    arg!(--reg_url <REGISTRY_URL> "The distribution server")
                        .default_value("http://localhost:6000"),
                )
                .arg(arg!(--reg_userpass <REGISTRY_USERPASS> "The user+password to authenticate against the distribution server in the format user:password"))
                .arg(arg!(-c --count <COUNT> "The amount of images to push").default_value("1"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("pull-images")
                .about("Pulls OCIs from a distribution server")
                .arg(
                    arg!(--reg_url <REGISTRY_URL> "The distribution server")
                        .default_value("https://index.docker.io"),
                )
                .arg(arg!(--reg_userpass <REGISTRY_USERPASS> "The user+password to authenticate against the distribution server in the format user:password"))
                .arg(arg!(-c --count <COUNT> "The amount of pulls").default_value("1"))
                .arg(arg!(-i --image <IMAGE> "The image to pull").default_value("alpine:latest"))
                .arg_required_else_help(false),
        )
}

fn parse_userpass(userpass: &str) -> (String, String) {
    let parts: Vec<&str> = userpass.split(':').collect();
    let username = parts[0].to_string();
    let password = parts[1].to_string();
    (username, password)
}

fn parse_reg(registry_url: &str) -> Result<(String, ClientProtocol)> {
    let reg_url =
        url::Url::parse(registry_url).context("failed to parse the url: {registry_url}")?;
    let reg_host = reg_url
        .host_str()
        .ok_or(anyhow!("url missing host: {registry_url}"))?;

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
        _ => bail!("unknown protocol: {reg_protocol}"),
    };

    Ok((reg, protocol))
}

async fn pull_images(matches: &ArgMatches) -> Result<()> {
    let count = matches
        .get_one::<String>("count")
        .ok_or(anyhow!("Count must be provided"))?;

    let count = count
        .parse::<usize>()
        .context("Count must be a valid number: {count}")?;

    let registry_url = matches
        .get_one::<String>("reg_url")
        .ok_or(anyhow!("Registry URL must be provided"))?;

    let (reg, protocol) =
        parse_reg(registry_url).context("couldn't parse the reg url: {registry_url}")?;

    let image = matches
        .get_one::<String>("image")
        .ok_or(anyhow!("Image must be provided"))?;

    let image = Reference::try_from(format!("{reg}/{image}"))
        .context("failed to parse the image: {image}")?;

    let mut auth = RegistryAuth::Anonymous;
    let userpass: Option<String> = matches.get_one::<String>("reg_userpass").cloned();
    if let Some(userpass) = userpass {
        let (user, password) = parse_userpass(&userpass);
        auth = RegistryAuth::Basic(user, password);
    }

    info!(
        image = image.whole(),
        count = count,
        registry_url = registry_url,
        "Pulling images"
    );

    let results = oci_distribution_tester::load_test_pull(count, image, auth, protocol).await;

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
    Ok(())
}

async fn push_images(matches: &ArgMatches) -> Result<()> {
    let count = matches
        .get_one::<String>("count")
        .ok_or(anyhow!("Count must be provided"))?;

    let count = count
        .parse::<usize>()
        .context("Count must be a valid number: {count}")?;

    let registry_url = matches
        .get_one::<String>("reg_url")
        .ok_or(anyhow!("Registry URL must be provided"))?;

    let (reg, protocol) =
        parse_reg(registry_url).context("couldn't parse the reg url: {registry_url}")?;

    let mut auth = RegistryAuth::Anonymous;
    let userpass: Option<String> = matches.get_one::<String>("reg_userpass").cloned();
    if let Some(userpass) = userpass {
        let (user, password) = parse_userpass(&userpass);
        auth = RegistryAuth::Basic(user, password);
    }

    info!(count = count, registry_url = registry_url, "Pushing images");

    let results = oci_distribution_tester::load_test_push(count, reg, auth, protocol).await;

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
    Ok(())
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    let verbose: bool = *matches
        .get_one("verbose")
        .ok_or_else(|| anyhow!("verbose not set"))?;

    let mut filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    if verbose {
        filter = EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into());
    }

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .map_err(|e| {
            eprintln!("Failed to initialize tracing: {e}");
            anyhow!(e)
        })?;

    let res = match matches.subcommand() {
        Some(("pull-images", matches)) => pull_images(matches).await,
        Some(("push-images", matches)) => push_images(matches).await,
        _ => unreachable!(),
    };
    if let Err(e) = &res {
        error!("{e}");
    }
    res
}
