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
    let string = "hello world";
    let encoded_data = encoder(String::from(string)).unwrap();
    let ecc_words = reed_solomon(encoded_data);
    println!("{ecc_words:?}");
    let data: Vec<u8> = vec![
        10, 201, 158, 214, 198, 69, 106, 119, 236, 198, 189, 241, 55, 96, 77, 236, 35, 52, 244, 55,
        96, 77, 236, 35, 52, 244,
    ];

    let matrix = module_placement(data);
    // data_masking(matrix);
    let data_with_mask = data_masking(matrix);
    let version_bits = version_bits("L", 4);
    let final_matrix = place_version_bits(data_with_mask, version_bits);
    image_generation(final_matrix);
}
