use std::fmt::Display;

use futures::future;
use oci_distribution::{
    client::{ClientProtocol, PushResponse},
    errors::OciDistributionError,
    secrets::RegistryAuth,
    Reference,
};
use tracing::{debug, instrument};

use crate::{
    client,
    fake::{self, MEGABYTE},
};

pub enum LoadTestError {
    OciDistributionError(OciDistributionError),
    JoinError(tokio::task::JoinError),
}

impl Display for LoadTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadTestError::OciDistributionError(e) => write!(f, "OciDistributionError: {e}"),
            LoadTestError::JoinError(e) => write!(f, "JoinError: {e}"),
        }
    }
}

/// Load tests a registry by pushing images to it.
#[instrument(skip(auth, protocol), level = "debug")]
pub async fn load_test_push(
    image_count: usize,
    host: String,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Vec<Result<PushResponse, LoadTestError>> {
    let mut handles = Vec::new();

    for i in 0..image_count {
        debug!("Kicking off push for image {i}");
        let h = tokio::task::spawn(push_reg_image(
            i,
            host.clone(),
            auth.clone(),
            protocol.clone(),
        ));
        handles.push(h);
    }
    debug!("Waiting for all pushes to complete");
    let results = future::join_all(handles).await;
    let results: Vec<Result<PushResponse, LoadTestError>> = results
        .into_iter()
        .map(|r| {
            r.map_err(LoadTestError::JoinError)
                .and_then(|r| r.map_err(LoadTestError::OciDistributionError))
        })
        .collect();
    results
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

/// Load tests a registry by pulling an image from it.
#[instrument(skip(auth, protocol), level = "debug")]
pub async fn load_test_pull(
    image_count: usize,
    image: Reference,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Vec<Result<(), LoadTestError>> {
    let mut handles = Vec::new();

    for _ in 0..image_count {
        debug!("Kicking off pull for image {image}");
        let h = tokio::task::spawn(pull_reg_image(
            image.clone(),
            auth.clone(),
            protocol.clone(),
        ));
        handles.push(h);
    }
    debug!("Waiting for all pulls to complete");
    let results = future::join_all(handles).await;
    let results: Vec<Result<(), LoadTestError>> = results
        .into_iter()
        .map(|r| {
            r.map_err(LoadTestError::JoinError)
                .and_then(|r| r.map_err(LoadTestError::OciDistributionError))
        })
        .collect();
    results
}

#[instrument(level = "debug", skip(auth, protocol))]
async fn pull_reg_image(
    image: Reference,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Result<(), OciDistributionError> {
    crate::client::pull_image(protocol, image, auth).await?;
    Ok(())
}

pub async fn push_image_index(
    image: Reference,
    auth: RegistryAuth,
    protocol: ClientProtocol,
) -> Result<String, OciDistributionError> {
    let index = fake::gen_oci_image_index();
    client::push_image_list(image, index, &auth, protocol).await
}
