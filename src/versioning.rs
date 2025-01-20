use crate::{
    module_placement::display,
    reed_solomon::polynomials::{init_tables, multiply_polynomials},
};

pub(crate) fn version_bits(error_correction: &str, mask_number: u8) -> Vec<u8> {
    let error_correction_bits: u8 = match error_correction {
        "L" => 1,
        "M" => 0,
        "Q" => 3,
        "H" => 2,
        _ => 10,
    };

    let mask_bits = mask_number;
    let format: u8 = mask_bits | (error_correction_bits << 3);
    let mut format_vec: Vec<u8> = (0..5).rev().map(|i| ((format >> i) & 1) as u8).collect();
    let mut result = [format_vec.clone(), vec![0; 10]].concat();
    println!("{:?}", result);
    while result[0] == 0 && result.len() > 10 {
        result.remove(0);
    }
    let mut generator: Vec<u8> = vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1];

    while result.len() > 10 {
        let padding = result.len() - generator.len();
        let new_generator = [generator.clone(), vec![0; padding]].concat();
        result = xor_polynomials(&new_generator, &result);
        while result.len() > 10 && result[0] == 0 {
            result.remove(0);
        }
        println!("{result:?}");
    }
    println!("{result:?}");
    result = [format_vec, result].concat();
    println!("{result:?}");

    result = xor_polynomials(&result, &vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0]);
    println!("{result:?}");

    result
}

pub(crate) fn xor_polynomials(p: &Vec<u8>, q: &Vec<u8>) -> Vec<u8> {
    let mut result = vec![0; p.len()];

    for i in 0..p.len() {
        result[i] = p[i] ^ q[i];
    }

    result
}

pub(crate) fn place_version_bits(data: Vec<u8>, version_bits: Vec<u8>) -> Vec<u8> {
    let n = 21;
    let mut result = data;
    for i in 0..15 {
        if i < 6 {
            result[8 * n + i] = version_bits[i];
            result[(n - 1 - i) * n + 8] = version_bits[i];
        }
        if i == 6 {
            result[8 * n + i + 1] = version_bits[i];
            result[(n - 1 - i) * n + 8] = version_bits[i];
        }
        if i == 7 {
            result[8 * n + i + 1] = version_bits[i];
            result[8 * n + n + 15 - i] = version_bits[i];
        }
        if i == 8 {
            result[7 * n + i] = version_bits[i];
            result[8 * n + n + 15 - i] = version_bits[i];
        }
        if i > 8 {
            result[8 * n + (n - (15 - i))] = version_bits[i];
            result[(14 - i) * n + 8] = version_bits[i];
        }
    }
    println!("\n Final matrix is:");
    display(&result, n);
    result
}
