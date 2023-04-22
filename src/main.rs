use std::env;

use oci_distribution::{
    client::{ClientProtocol, PushResponse},
    Reference,
};
use registry_load_tester::fake::MEGABYTE;
use tracing::{debug, info, instrument};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[instrument]
async fn push_fake() {
    let layers = registry_load_tester::fake::gen_rand_layers(10 * MEGABYTE, 10);
    let image = registry_load_tester::fake::gen_image(layers).unwrap();

    let reference: Reference = "lswith/test:latest".parse().unwrap();

    let user = env::var("DOCKER_USER").unwrap();
    let password = env::var("DOCKER_PASSWORD").unwrap();
    let auth =
        oci_distribution::secrets::RegistryAuth::Basic(user.to_string(), password.to_string());

    info!("pushing image");

    let resp: PushResponse = registry_load_tester::client::push_image(
        image.layers,
        image.config,
        reference,
        image.manifest,
        &auth,
        ClientProtocol::Https,
    )
    .await
    .unwrap();

    debug!("{}", resp.manifest_url);
}

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
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .unwrap();

    // push_fake().await;
    // pull_docker_reg_push_local().await
    // pull_local_push_docker_reg().await
    pull_local_push_local().await
}
