use module_placement::module_placement;
use reed_solomon::reed_solomon;
mod encoder;
mod module_placement;
mod reed_solomon;

fn main() {
<<<<<<< HEAD
    // TODO: Error management
    let string = "hello world";
    let encoded_data = encoder(String::from(string)).unwrap();
    let ecc_words = reed_solomon(encoded_data);
    println!("{ecc_words:?}");
    let data: Vec<u8> = vec![
        10, 201, 158, 214, 198, 69, 106, 119, 236, 198, 189, 241, 55, 96, 77, 236, 35, 52, 244,
    ];

    module_placement(data);
=======
    // println!("{:?}", reed_solomon());
    // encoder(String::from("hello world"))
    // let result = reed_solomon(vec![
    //     17, 236, 17, 236, 17, 236, 64, 67, 77, 220, 114, 209, 120, 11, 91, 32,
    // ]);
    // println!("result: {:?}", result);
    // println!("{}", 256 ^ 285)
    let data: Vec<u8> = vec![
        10, 201, 158, 214, 198, 69, 106, 119, 236, 198, 189, 241, 55, 96, 77, 236, 35, 52, 244, 55,
        96, 77, 236, 35, 52, 244,
    ];

    module_placement(data);
>>>>>>> 5eb6a7e (filling first columns)
}
