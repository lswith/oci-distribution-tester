//! CLI for testing OCI distribution servers
use crate::tester::{load_test_pull, load_test_push};
use anyhow::{anyhow, bail, Context, Result};
use oci_distribution::{client::ClientProtocol, secrets::RegistryAuth, Reference};
use tracing::{error, info};

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

/// Pulls images from a registry.
///
/// # Errors
///
/// * If the image is not valid
/// * If the registry URL is not valid
pub async fn pull_images(
    reg_url: String,
    count: usize,
    reg_userpass: Option<String>,
    image: String,
) -> Result<()> {
    let (reg, protocol) = parse_reg(&reg_url).context("couldn't parse the reg url: {reg_url}")?;

    let image = Reference::try_from(format!("{reg}/{image}"))
        .context("failed to parse the image: {image}")?;

    let mut auth = RegistryAuth::Anonymous;
    if let Some(userpass) = reg_userpass {
        let (user, password) = parse_userpass(&userpass);
        auth = RegistryAuth::Basic(user, password);
    }

    info!(
        image = image.whole(),
        count = count,
        registry_url = reg_url,
        "Pulling images"
    );

    let results = load_test_pull(count, image, auth, protocol).await;

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

/// Pushes images to a registry.
///
/// # Errors
/// * If the count is not a valid number
/// * If the registry URL is not provided
/// * If the registry URL is not valid
pub async fn push_images(
    reg_url: String,
    count: usize,
    reg_userpass: Option<String>,
    namespace: String,
    image: String,
    tag: String,
) -> Result<()> {
    let (reg, protocol) = parse_reg(&reg_url).context("couldn't parse the reg url: {reg_url}")?;

    let mut auth = RegistryAuth::Anonymous;
    if let Some(userpass) = reg_userpass {
        let (user, password) = parse_userpass(&userpass);
        auth = RegistryAuth::Basic(user, password);
    }

    info!(count = count, registry_url = reg_url, "Pushing images");

    let results = load_test_push(count, reg, auth, protocol, namespace, image, tag).await;

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
/// Pushes an image index to a registry.
///
/// # Errors
/// * If the registry URL is not valid
/// * If the image is not valid
pub async fn push_image_index(
    reg_url: String,
    reg_userpass: Option<String>,
    image: String,
) -> Result<()> {
    let (reg, protocol) = parse_reg(&reg_url).context("couldn't parse the reg url: {reg_url}")?;

    let mut auth = RegistryAuth::Anonymous;
    if let Some(userpass) = reg_userpass {
        let (user, password) = parse_userpass(&userpass);
        auth = RegistryAuth::Basic(user, password);
    }

    info!(registry_url = reg_url, "Pushing image list");

    let reference: Reference = format!("{reg}/{image}")
        .parse()
        .context("couldn't create a reference from {reg}/{image}")?;
    match crate::tester::push_image_index(reference, auth, protocol).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Ok(())
        }
    }
}
