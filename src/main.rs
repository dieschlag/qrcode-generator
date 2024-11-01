use encoder::encoder;
use reed_solomon::reed_solomon;

mod encoder;
mod reed_solomon;

fn main() {
    // TODO: Error management
    let string = "hello world";
    encoder(String::from(string));
}
