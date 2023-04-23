//! # Load test an OCI compliant registry
use std::env;

use oci_distribution::{client::ClientProtocol, secrets::RegistryAuth};
use tracing::{error, info};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[tokio::main]
async fn main() {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .unwrap();

    let thread_count = env::var("THREAD_COUNT")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap();

    let reg_url = env::var("REGISTRY_URL").unwrap_or("http://localhost:6000".to_string());
    let reg_url = url::Url::parse(&reg_url).unwrap();
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

    let user = env::var("DOCKER_USER");
    let password = env::var("DOCKER_PASSWORD");

    let mut auth = RegistryAuth::Anonymous;

    if let Ok(user) = user {
        if let Ok(password) = password {
            auth = RegistryAuth::Basic(user, password);
        }
    }

    info!("Starting load test with {thread_count} threads");
    let results = registry_tester::load_test(thread_count, reg, auth, protocol).await;

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
