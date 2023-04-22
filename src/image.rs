use std::fmt::Formatter;

use oci_distribution::{
    client::{self, ImageLayer},
    manifest::OciImageManifest,
};

#[derive(Clone)]
pub struct Image {
    pub manifest: Option<OciImageManifest>,
    pub config: client::Config,
    pub layers: Vec<ImageLayer>,
    pub digest: Option<String>,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("manifest", &self.manifest)
            .field("digest", &self.digest)
            .finish()
    }
}
