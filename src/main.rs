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

    // ------------------------------------------------
    //
    // For now, it is only possible to generate 1L QR Code. To generate a QR code, update the string bellow,
    // and verify it is possible to generate a 1L QR code with it.
    //
    // ------------------------------------------------

    let input = "hello";

    let encoded_data = encoder(String::from(input)).unwrap();

    let ecc_data = reed_solomon(encoded_data.clone(), 7);

    // println!("{ecc_data:?}");
    // println!("");

    // let full_data = [encoded_data, ecc_data].concat();

    // println!("Full data in bits");

    // for bit in full_data.clone() {
    //     print!("{bit:08b}");
    // }

    // let matrix = module_placement(full_data);
    // let (data_with_mask, mask_number) = data_masking(matrix);
    // let version_bits = version_bits("L", mask_number);
    // let final_matrix = place_version_bits(data_with_mask, version_bits);

    // image_generation(final_matrix);
}
