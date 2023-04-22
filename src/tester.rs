use futures::future;
use oci_distribution::{
    client::{ClientProtocol, PushResponse},
    errors::OciDistributionError,
    secrets::RegistryAuth,
    Reference,
};
use tracing::{info, instrument};

use crate::fake::MEGABYTE;

#[instrument(skip(auth, protocol))]
pub async fn load_test(
    image_count: usize,
    reg: String,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Result<(), tokio::task::JoinError> {
    let mut handles = Vec::new();

    for i in 0..image_count {
        info!("Kicking off push for image {i}");
        let h = tokio::task::spawn(push_reg_image(
            i,
            reg.clone(),
            auth.clone(),
            protocol.clone(),
        ));
        handles.push(h);
    }
    info!("Waiting for all pushes to complete");
    future::try_join_all(handles).await?;
    Ok(())
}

#[instrument(level = "debug", skip(auth, protocol))]
async fn push_reg_image(
    i: usize,
    reg: String,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Result<PushResponse, OciDistributionError> {
    let layers = crate::fake::gen_rand_layers(10 * MEGABYTE, 1);
    let image = crate::fake::gen_image(layers).unwrap();

    let reference: Reference = format!("{reg}/test/this-{i}:latest").parse().unwrap();

    let res = crate::client::push_image(
        image.layers,
        image.config,
        reference,
        image.manifest,
        &auth,
        protocol,
    )
    .await?;
    Ok(res)
}
