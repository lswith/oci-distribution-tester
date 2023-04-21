use rand::RngCore;
use tar::{Builder, Header};

pub fn fake_image_layer(data: Vec<u8>) -> oci_distribution::client::ImageLayer {
    oci_distribution::client::ImageLayer::oci_v1(data, None)
}

pub fn fake_tar_data() -> Vec<u8> {
    let data: &mut [u8] = &mut [0; 1028];

    rand::thread_rng().fill_bytes(data);

    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_cksum();

    let mut ar = Builder::new(Vec::new());

    ar.append_data(&mut header, "really/long/path/to/foo", data.as_ref())
        .unwrap();

    ar.into_inner().unwrap()
}
