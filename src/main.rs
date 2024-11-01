use encoder::encoder;
use reed_solomon::reed_solomon;
mod encoder;
mod reed_solomon;

fn main() {
    // TODO: Error management
    let string = "hello world";
    let _encoded_data = encoder(String::from(string));
    let mut test = vec![
        17, 236, 17, 236, 17, 236, 64, 67, 77, 220, 114, 209, 120, 11, 91, 32,
    ];
    test.reverse();
    let ecc_words = reed_solomon(test);
    println!("{ecc_words:?}");
}
