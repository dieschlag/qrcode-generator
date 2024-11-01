use encoder::encoder;

mod encoder;

fn main() {
    // TODO: Error management
    let string = "hello world";
    encoder(String::from(string));
}
