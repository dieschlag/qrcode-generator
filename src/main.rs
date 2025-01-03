use encoder::encoder;
use module_placement::module_placement;
use reed_solomon::reed_solomon;

mod encoder;
mod module_placement;
mod reed_solomon;

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

    module_placement(data);
}
