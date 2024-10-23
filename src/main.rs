use encoder::encoder;
use reed_solomon::reed_solomon;

mod encoder;
mod reed_solomon;

fn main() {
    // println!("{:?}", reed_solomon());
    // encoder(String::from("hello world"))
    let result = reed_solomon(vec![
        17, 236, 17, 236, 17, 236, 64, 67, 77, 220, 114, 209, 120, 11, 91, 32,
    ]);
    println!("result: {:?}", result);
    // println!("{}", 256 ^ 285)
}
