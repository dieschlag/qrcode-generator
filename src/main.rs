use data_masking::data_masking;
use encoder::encoder;
use image_generation::image_generation;
use module_placement::module_placement;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
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
    let (data_with_mask, mask_number) = data_masking(matrix);
    let version_bits = version_bits("L", mask_number);
    let final_matrix = place_version_bits(data_with_mask, version_bits);
    image_generation(final_matrix);

    let data = "hello world"; // Les données à encoder dans le QR Code
    let level = EcLevel::L; // Niveau de correction : L, M, Q, ou H

    // Générer le QR code
    let code =
        QrCode::with_error_correction_level(data, level).expect("Failed to generate QR code");

    // Rendu en SVG
    let image = code
        .render::<svg::Color>()
        .min_dimensions(200, 200) // Dimensions du QR Code
        .dark_color(svg::Color("#000000")) // Couleur des pixels
        .light_color(svg::Color("#ffffff")) // Couleur de l'arrière-plan
        .build();

    // Sauvegarder le QR Code dans un fichier SVG
    std::fs::write("qrcode.svg", image).expect("Failed to save QR code");
    println!("QR code saved as qrcode.svg");
}
