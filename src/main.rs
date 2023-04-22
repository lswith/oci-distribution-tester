use std::env;

use oci_distribution::{
    client::{ClientProtocol, PushResponse},
    secrets::RegistryAuth,
    Reference,
};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[instrument]
async fn pull_docker_reg_push_docker_reg() {
    let user = env::var("DOCKER_USER").unwrap();
    let password = env::var("DOCKER_PASSWORD").unwrap();
    let auth =
        oci_distribution::secrets::RegistryAuth::Basic(user.to_string(), password.to_string());

    let image_ref = "alpine:latest".parse().unwrap();

    info!("pulling image {image_ref}");
    let image = registry_load_tester::client::pull_image(ClientProtocol::Https, image_ref, &auth)
        .await
        .unwrap();

    debug!("got image {image:?}");

    let reference: Reference = "lswith/alpine:latest".parse().unwrap();

    let mut manifest = image.manifest.unwrap();
    manifest.media_type = Some(oci_distribution::manifest::OCI_IMAGE_MEDIA_TYPE.to_string());

    info!("pushing image");

    let resp: PushResponse = registry_load_tester::client::push_image(
        image.layers,
        image.config,
        reference,
        Some(manifest),
        &auth,
        ClientProtocol::Https,
    )
    .await
    .unwrap();

    debug!("{}", resp.manifest_url);
}
#[instrument]
async fn pull_local_push_docker_reg() {
    let image_ref = "localhost:6000/test/this:old".parse().unwrap();

    info!("pulling image {image_ref}");
    let image = registry_load_tester::client::pull_image(
        ClientProtocol::Http,
        image_ref,
        &oci_distribution::secrets::RegistryAuth::Anonymous,
    )
    .await
    .unwrap();

    debug!("got image {image:?}");

    let reference: Reference = "lswith/test:latest".parse().unwrap();

    let user = env::var("DOCKER_USER").unwrap();
    let password = env::var("DOCKER_PASSWORD").unwrap();
    let auth =
        oci_distribution::secrets::RegistryAuth::Basic(user.to_string(), password.to_string());

    info!("pushing image");
    let mut manifest = image.manifest.unwrap();
    manifest.media_type = Some(oci_distribution::manifest::OCI_IMAGE_MEDIA_TYPE.to_string());

    let resp: PushResponse = registry_load_tester::client::push_image(
        image.layers,
        image.config,
        reference,
        Some(manifest),
        &auth,
        ClientProtocol::Https,
    )
    .await
    .unwrap();

    debug!("{}", resp.manifest_url);
}

#[instrument]
async fn pull_docker_reg_push_local() {
    let user = env::var("DOCKER_USER").unwrap();
    let password = env::var("DOCKER_PASSWORD").unwrap();
    let auth =
        oci_distribution::secrets::RegistryAuth::Basic(user.to_string(), password.to_string());

    let image_ref = "alpine:latest".parse().unwrap();

    info!("pulling image {image_ref}");
    let image = registry_load_tester::client::pull_image(ClientProtocol::Https, image_ref, &auth)
        .await
        .unwrap();

    debug!("got image {image:?}");

    let reference: Reference = "localhost:6000/test/this:old".parse().unwrap();

    let mut manifest = image.manifest.unwrap();
    manifest.media_type = Some(oci_distribution::manifest::OCI_IMAGE_MEDIA_TYPE.to_string());

    info!("pushing image");

    let resp: PushResponse = registry_load_tester::client::push_image(
        image.layers,
        image.config,
        reference,
        Some(manifest),
        &oci_distribution::secrets::RegistryAuth::Anonymous,
        ClientProtocol::Http,
    )
    .await
    .unwrap();

    debug!("{}", resp.manifest_url);
}

#[instrument]
async fn pull_local_push_local() {
    let image_ref = "localhost:6000/test/this:old".parse().unwrap();

    info!("pulling image {image_ref}");
    let image = registry_load_tester::client::pull_image(
        ClientProtocol::Http,
        image_ref,
        &oci_distribution::secrets::RegistryAuth::Anonymous,
    )
    .await
    .unwrap();

    debug!("got image {image:?}");

    let image_ref = "localhost:6000/test/this:new".parse().unwrap();

    info!("pushing image");
    let mut manifest = image.manifest.unwrap();
    manifest.media_type = Some(oci_distribution::manifest::OCI_IMAGE_MEDIA_TYPE.to_string());

    let resp: PushResponse = registry_load_tester::client::push_image(
        image.layers,
        image.config,
        image_ref,
        Some(manifest),
        &oci_distribution::secrets::RegistryAuth::Anonymous,
        ClientProtocol::Http,
    )
    .await
    .unwrap();

    debug!("{}", resp.manifest_url);
}

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
    let res = registry_load_tester::tester::load_test(thread_count, reg, auth, protocol).await;
    if let Err(e) = res {
        error!("{}", e);
    }
}
