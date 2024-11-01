use encoder::encoder;

mod encoder;

fn main() {
    // TODO: Error management
    let string = "hello world";
    let _ = encoder(String::from(string));
}
