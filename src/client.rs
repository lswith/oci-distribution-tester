use oci_distribution::{
    client::{ClientConfig, ClientProtocol, Config, ImageLayer, PushResponse},
    errors::OciDistributionError,
    manifest::OciImageManifest,
    secrets::RegistryAuth,
    Reference,
};
use tracing::instrument;

use crate::image::Image;

#[instrument(level = "trace", err)]
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

    Ok(Image {
        manifest: image.manifest,
        config: image.config,
        layers: image.layers,
        digest: image.digest,
    })
}

#[instrument(level = "trace", skip(layers, config), err)]
pub async fn push_image(
    layers: Vec<ImageLayer>,
    config: Config,
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

    client.push(&image, layers, config, auth, manifest).await
}
