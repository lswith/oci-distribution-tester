use oci_distribution::{
    client::ImageLayer,
    config::{Architecture, ConfigFile, Os},
    errors::OciDistributionError,
};
use rand::{distributions::Alphanumeric, Rng, RngCore};
use std::{
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
};
use tar::{Builder, Header};

use crate::image::Image;

pub const MEGABYTE: usize = 1024 * 1024;

pub fn gen_tar_image_layer(size: usize) -> oci_distribution::client::ImageLayer {
    let filename = gen_file_name(10);
    let filepath = gen_file_path(3);
    let data = gen_file_data(size);
    let tar_data = TarData::new(&filename, &filepath, data);
    oci_distribution::client::ImageLayer::oci_v1(tar_data.to_vec(), None)
}

pub fn gen_gzip_tar_image_layer(size: usize) -> oci_distribution::client::ImageLayer {
    let filename = gen_file_name(10);
    let filepath = gen_file_path(3);
    let data = gen_file_data(size);
    let tar_data = TarData::new(&filename, &filepath, data);
    let mut gz_data = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gz_data.write_all(&tar_data.to_vec()).unwrap();
    let gz_data = gz_data.finish().unwrap();
    oci_distribution::client::ImageLayer::oci_v1_gzip(gz_data, None)
}

pub fn gen_rand_layers(size: usize, count: usize) -> Vec<oci_distribution::client::ImageLayer> {
    let mut layers = Vec::with_capacity(count);
    for _ in 0..count {
        if rand::random::<f32>() > 0.5 {
            layers.push(gen_gzip_tar_image_layer(size));
        } else {
            layers.push(gen_tar_image_layer(size));
        }
    }
    layers
}

pub fn gen_image(layers: Vec<ImageLayer>) -> Result<Image, OciDistributionError> {
    let config_file = ConfigFile {
        os: Os::Linux,
        architecture: Architecture::Amd64,
        ..Default::default()
    };

    let config = oci_distribution::client::Config::oci_v1_from_config_file(config_file, None)?;

    let mut manifest =
        oci_distribution::manifest::OciImageManifest::build(layers.as_ref(), &config, None);
    manifest.media_type = Some(oci_distribution::manifest::OCI_IMAGE_MEDIA_TYPE.to_string());

    let digest = manifest.config.digest.clone();

    Ok(Image {
        manifest: Some(manifest),
        config,
        layers,
        digest: Some(digest),
    })
}

pub fn gen_file_data(size: usize) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(size);
    rand::thread_rng().fill_bytes(&mut data);
    data
}

pub fn gen_file_name(size: usize) -> String {
    let filename: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    filename
}

pub fn gen_file_path(segments: usize) -> PathBuf {
    let mut path = PathBuf::new();
    for _ in 0..segments {
        path.push(gen_file_name(10));
    }
    path
}

pub struct TarData(Vec<u8>);

impl TarData {
    pub fn new(filename: &str, tarpath: &Path, data: Vec<u8>) -> TarData {
        let mut header = Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_cksum();

        let mut ar = Builder::new(Vec::new());

        let p = tarpath.join(filename);

        ar.append_data(&mut header, &p, data.deref()).unwrap();

        Self(ar.into_inner().unwrap())
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
}
