/// We work with numbers considered inside the Galois Field GF(256)
/// and since them with bit-wise modulo 2 arithmetic, the result of
/// an addition or substraction in that field is equivalent to a XOR.
///
/// All numbers inside GF(256) can be represented using powers of 2,
/// so any element a can be written as a = 2^i.
///
/// The QR Code specification specifies to use byte-wise modulo
/// 100011101, which is 285. So when a number is larger than 256, it
/// should be XORed with 285. It allows all numbers to stay between 0
/// and 255.
///
///     

/// Initilaizes logarithmic and anti-logarithmic tables in GF(256)
/// Log and anti-log tables work such that if we have a^i = x, then:
///     log[x] = i ; antilog[i] = x
pub(crate) fn init_tables() -> (Vec<u8>, Vec<u8>) {
    // Initialization
    let mut log_table = vec![0; 256];
    let mut antilog_table = vec![0; 256];
    let mut x = 1;
    let primitive_polynomial: u16 = 285; // Qr code requires working modulo 285 in GF(256)

    // Filling the tables
    for i in 0..255 {
        log_table[x as usize] = i as u8;
        antilog_table[i] = x;

        // To avoid overflow, convert x as u_16, if result is superior to 255, then XOR is applied
        let mut x_u16 = (x as u16) << 1;
        if x_u16 > 255 {
            x_u16 ^= primitive_polynomial as u16;
        }

        x = x_u16 as u8;
    }

    // To have 0 = 255 in GF(256)
    antilog_table[255] = antilog_table[0];

    (log_table, antilog_table)
}

/// Mutliplies two elements on GF(256) using log tables
pub(crate) fn multiply(a: u8, b: u8, log_table: &Vec<u8>, antilog_table: &Vec<u8>) -> u8 {
    if a == 0 || b == 0 {
        0
    } else {
        // Calculating log of result modulo 255 to stay in GF(256)
        let log_a = log_table[a as usize] as usize;
        let log_b = log_table[b as usize] as usize;
        let log_result = (log_a + log_b) % 255;

        // Finding result in antilog_table
        antilog_table[log_result]
    }
}

/// Mutliplies two polynomials reprensented by vectors of GF(256) elements.
/// Vectors are such that first element is the main term of the polynomial.
/// Result is another vector.
pub(crate) fn multiply_polynomials(
    p: &Vec<u8>,
    q: &Vec<u8>,
    log_table: &Vec<u8>,
    antilog_table: &Vec<u8>,
) -> Vec<u8> {
    let mut result = vec![0; p.len() + q.len() - 1]; // Le degré max du produit est p.len() + q.len() - 2

    // Multiplier chaque coefficient de p avec chaque coefficient de q
    for (i, &p_coef) in p.iter().enumerate() {
        for (j, &q_coef) in q.iter().enumerate() {
            result[i + j] ^= multiply(p_coef, q_coef, &log_table, &antilog_table)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_and_antilogs() {
        let (log_table, antilog_table) = init_tables();
        println!("{log_table:?}");
        assert_eq!(
            antilog_table,
            vec![
                1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90,
                180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148,
                53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93,
                186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202,
                137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223,
                163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208,
                189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51,
                102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84,
                168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209,
                191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171,
                75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200,
                141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249,
                239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247,
                243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108,
                216, 173, 71, 142, 1
            ]
        );
        assert_eq!(
            log_table,
            vec![
                0, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14,
                52, 141, 239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36,
                15, 33, 53, 147, 142, 218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185,
                201, 154, 9, 120, 77, 228, 114, 166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152,
                37, 179, 16, 145, 34, 136, 54, 208, 148, 206, 143, 150, 219, 189, 241, 210, 19, 92,
                131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126, 110, 107, 58, 40, 84, 250, 133,
                186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172, 115, 243, 167, 87,
                7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24, 227, 165,
                153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91,
                149, 188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20,
                42, 93, 158, 132, 60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216, 183, 123, 164,
                118, 196, 23, 73, 236, 127, 12, 111, 246, 108, 161, 59, 82, 41, 157, 85, 170, 251,
                96, 134, 177, 187, 204, 62, 90, 203, 89, 95, 176, 156, 169, 160, 81, 11, 245, 22,
                235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234,
                168, 80, 88, 175
            ]
        )
    }
}
