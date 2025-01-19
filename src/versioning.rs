use crate::reed_solomon::polynomials::{init_tables, multiply_polynomials};

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
    let mut seen_one = false;
    let mut result: Vec<u8> = (0..5)
        .rev()
        .filter_map(|i| {
            let bit = ((format >> i) & 1) as u8;
            if seen_one || bit == 1 {
                seen_one = true; // Une fois qu'un '1' est vu, on commence à tout inclure
                Some(bit)
            } else {
                None
            }
        })
        .collect();
    result = [result, vec![0; 10]].concat();
    println!("{:?}", result);
    let mut generator: Vec<u8> = vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1];
    let (log_table, antilog_table) = init_tables();

    while result.len() > 10 {
        let padding = result.len() - generator.len();
        let new_generator = [generator.clone(), vec![0; padding]].concat();
        result = multiply_polynomials(&new_generator, &result, &log_table, &antilog_table);
        while result.len() > 10 && result[0] == 0 {
            result.remove(0);
        }
    }
    vec![]
}
