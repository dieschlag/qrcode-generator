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

// pub(crate) fn add(a: u8, b: u8) -> u8 {
//     a ^ b
// }

/// Mutliplies two polynomials reprensented by vectors of GF(256) elements.
/// Vectors are such that last element is the main term of the polynomial.
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
