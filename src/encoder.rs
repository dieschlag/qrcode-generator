use std::error::Error;

pub(crate) fn encoder(text: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let copy = text.clone().into_bytes();
    let mut bytes = text.clone().into_bytes();
    let length: u8 = text.len().try_into()?;
    bytes.insert(0, length);
    let mut result: Vec<u8> = Vec::new();

    let first_byte = bytes[0];
    let first_result_byte = (0b0100 << 4) | (first_byte >> 4);
    result.push(first_result_byte);

    for i in 0..bytes.len() - 1 {
        let current_byte = bytes[i];
        let next_byte = bytes[i + 1];
        let combined_byte = ((current_byte & 0b00001111) << 4) | (next_byte >> 4);
        result.push(combined_byte);
    }

    let last_byte = bytes[bytes.len() - 1];
    let last_combined_byte = (last_byte & 0b00001111) << 4;

    result.push(last_combined_byte);

    while result.len() < 19 {
        if let Some(&last) = result.last() {
            // Si le dernier élément est 236, ajouter 17
            if last == 236 {
                result.push(17);
            } else {
                result.push(236);
            }
        }
    }
    let mut single_line_bytes = String::new();
    println!("Print each byte \n");
    for bite in result {
        println!("{bite:08b}");
        single_line_bytes.push_str(&format!("{bite:08b}"));
    }

    println!("Print bits as single string \n");
    println!("{single_line_bytes}");

    println!("Compare the two :");

    let mut copy_single_line_bytes = String::new();
    for bite in copy {
        copy_single_line_bytes.push_str(&format!("{bite:08b}"));
    }

    println!("original: {copy_single_line_bytes}");
    println!("modified: {single_line_bytes}");

    return Ok(vec![]);
}
