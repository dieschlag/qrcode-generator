use data_masking::data_masking;
use encoder::encoder;
use image_generation::image_generation;
use module_placement::module_placement;
use reed_solomon::reed_solomon;
use versioning::{place_version_bits, version_bits};

mod data_masking;
mod encoder;
mod image_generation;
mod module_placement;
mod reed_solomon;
mod versioning;

fn main() {
    // TODO: Error management
    let string = "hello";
    let encoded_data = encoder(String::from(string)).unwrap();
    let ecc_data = reed_solomon(encoded_data.clone());
    println!("{ecc_data:?}");
    let full_data = [encoded_data, ecc_data].concat();
    let matrix = module_placement(full_data);
    // data_masking(matrix);
    let (data_with_mask, mask_number) = data_masking(matrix);
    let version_bits = version_bits("L", 0);
    let final_matrix = place_version_bits(data_with_mask, version_bits);
    image_generation(final_matrix);
}
