use encoder::encoder;
use reed_solomon::reed_solomon;

mod encoder;
mod reed_solomon;

fn main() {
    // println!("{:?}", reed_solomon());
    let result = reed_solomon(encoder(String::from("hello world")));
    println!("result: {:?}", result);
    // println!("{}", 256 ^ 285)
}
