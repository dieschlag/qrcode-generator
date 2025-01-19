pub(crate) fn version_bits(error_correction: &str, mask_number: u8) -> Vec<u8> {
    let result = 0 as u16;
    let error_correction_bits: u8 = match error_correction {
        "L" => 1,
        "M" => 0,
        "Q" => 3,
        "H" => 2,
        _ => 10,
    };

    let mask_bits = mask_number;
    let format: u8 = mask_bits | (error_correction_bits << 3);
    println!("{:08b}", format);
    vec![]
}
