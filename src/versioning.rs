pub(crate) fn version_bits(error_correction: &str, mask_number: u32) -> Vec<u8> {
    let error_correction_bits: u8 = match error_correction {
        "L" => 1,
        "M" => 0,
        "Q" => 3,
        "H" => 2,
        _ => 10,
    };

    let mask_pattern_bits
    vec![]
}
