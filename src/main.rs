use oci_distribution::{
    client::{ClientConfig, ClientProtocol},
    Reference,
};
use tracing::info;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[tokio::main]
async fn main() {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::TRACE.into());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .unwrap();

    info!("creating fake tar data");
    let tar = registry_load_tester::fake::fake_tar_data();
    info!("creating fake image layer");
    let layer = registry_load_tester::fake::fake_image_layer(tar);
    let layers = &[layer];
    info!("creating client");
    let mut client = oci_distribution::client::Client::new(ClientConfig {
        protocol: ClientProtocol::Http,
        ..ClientConfig::default()
    });
    let reference: Reference = "localhost:6000/test/this:latest".parse().unwrap();
    let config = oci_distribution::client::Config::oci_v1(b"{}".to_vec(), None);
    info!("creating manifest");
    let image_manifest = oci_distribution::manifest::OciImageManifest::build(layers, &config, None);

    let auth = oci_distribution::secrets::RegistryAuth::Anonymous;

    info!("pushing");
    client
        .push(&reference, layers, config, &auth, Some(image_manifest))
        .await
        .map(|push_response| push_response.manifest_url)
        .unwrap();
}
