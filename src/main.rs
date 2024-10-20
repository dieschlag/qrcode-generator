use encoder::encoder;
mod encoder;

fn main() {
    let error_correction = "L";
    let version = 1;
    let string = "hello world";
    encoder(String::from(string));
}
