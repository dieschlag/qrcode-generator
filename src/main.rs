use encoder::encoder;
use reed_solomon::reed_solomon;

mod encoder;
mod reed_solomon;

fn main() {
    let string = "hello world";
    encoder(String::from(string));
}
