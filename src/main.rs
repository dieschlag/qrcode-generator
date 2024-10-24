use reed_solomon::reed_solomon;
mod encoder;
mod reed_solomon;

fn main() {
    // TODO: Error management
    let string = "hello world";
    let encoded_data = encoder(String::from(string)).unwrap();
    let ecc_words = reed_solomon(encoded_data);
    println!("{ecc_words:?}");
}
