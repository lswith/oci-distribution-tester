use oci_distribution::{
    client::{ClientConfig, ClientProtocol, Config, ImageLayer, PushResponse},
    config::ConfigFile,
    errors::OciDistributionError,
    manifest::OciImageManifest,
    secrets::RegistryAuth,
    Reference,
};
use tracing::{debug, instrument};

use crate::image::Image;

#[instrument(err)]
pub async fn pull_image(
    protocol: ClientProtocol,
    image: Reference,
    auth: &RegistryAuth,
) -> Result<Image, OciDistributionError> {
    let mut client = oci_distribution::client::Client::new(ClientConfig {
        protocol,
        platform_resolver: Some(Box::new(oci_distribution::client::linux_amd64_resolver)),
        ..ClientConfig::default()
    });

    if *auth != RegistryAuth::Anonymous {
        client
            .auth(&image, auth, oci_distribution::RegistryOperation::Pull)
            .await?;
    }

    let image = client
        .pull(
            &image,
            auth,
            vec![
                oci_distribution::manifest::IMAGE_LAYER_MEDIA_TYPE,
                oci_distribution::manifest::IMAGE_LAYER_GZIP_MEDIA_TYPE,
                oci_distribution::manifest::IMAGE_DOCKER_LAYER_TAR_MEDIA_TYPE,
                oci_distribution::manifest::IMAGE_DOCKER_LAYER_GZIP_MEDIA_TYPE,
            ],
        )
        .await?;

    let d = String::from_utf8(image.config.data.clone()).unwrap();
    debug!("{}", d);
    let config: oci_distribution::config::ConfigFile = serde_json::from_str(&d).unwrap();
    debug!("{:?}", config);

    Ok(Image {
        manifest: image.manifest,
        config,
        layers: image.layers,
        digest: image.digest,
    })
}

#[instrument(skip(layers), err)]
pub async fn push_image(
    layers: Vec<ImageLayer>,
    config: ConfigFile,
    image: Reference,
    manifest: Option<OciImageManifest>,
    auth: &RegistryAuth,
    protocol: ClientProtocol,
) -> Result<PushResponse, OciDistributionError> {
    let mut client = oci_distribution::client::Client::new(ClientConfig {
        protocol,
        platform_resolver: Some(Box::new(oci_distribution::client::linux_amd64_resolver)),
        ..ClientConfig::default()
    });

    if *auth != RegistryAuth::Anonymous {
        client
            .auth(&image, auth, oci_distribution::RegistryOperation::Push)
            .await?;
    }

    let config = Config::oci_v1_from_config_file(config, None)?;

    client.push(&image, &layers, config, auth, manifest).await
}
