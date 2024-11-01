use std::error::Error;

pub(crate) fn encoder(text: String) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("**************************** Encoding Data ****************************\n");
    println!("Initial text data: {} \n", text);

    // Converting string to a sequence of bytes with length encoded in it
    let mut bytes = text.clone().into_bytes();
    let length: u8 = text.len().try_into()?; //Need to be a u8 for a 1L
    bytes.insert(0, length);

    // Initiate result vector, containing a stream of bites divided in u8, with length, method indicator and data
    let mut result: Vec<u8> = Vec::new();

    // Encoding first bits: 0100, indicating binary encoding is used
    let first_byte = bytes[0];
    // Pushing 0100 at the beginning of first octet with first four bits of first data octet
    let first_result_byte = (0b0100 << 4) | (first_byte >> 4);
    result.push(first_result_byte);

    // Shifting all bits in u8 since to compensate shift at the beggining
    for i in 0..bytes.len() - 1 {
        let current_byte = bytes[i];
        let next_byte = bytes[i + 1];
        let combined_byte = ((current_byte & 0b00001111) << 4) | (next_byte >> 4);
        result.push(combined_byte);
    }

    // Shifting last byte
    let last_byte = bytes[bytes.len() - 1];
    let last_combined_byte = (last_byte & 0b00001111) << 4;
    result.push(last_combined_byte);

    // Making sure that the length of final bit strean matches qrcode requirements (here 19 data words)
    // We complete the final sequence with the pattern 11101100 00010001, corresponding to 236 and 17 respectively
    while result.len() < 19 {
        if result.last().unwrap() == &236 {
            result.push(17);
        } else {
            result.push(236);
        }
    }

    // Displaying result as a single bit stream
    println!("Final encoding as a single bit stream: ");
    for byte in &result {
        print!("{byte:08b}");
    }
    println!();

    Ok(result)
}
