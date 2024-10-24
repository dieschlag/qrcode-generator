/// Initialise les tables logarithmiques et antilogarithmiques pour GF(256)
pub(crate) fn init_tables() -> (Vec<u8>, Vec<u8>) {
    let mut log_table = vec![0; 256];
    let mut antilog_table = vec![0; 256];

    let mut x = 1;
    let primitive_polynomial: u16 = 285; // Polynôme irréductible pour GF(256) (x^8 + x^4 + x^3 + x^2 + 1)

    for i in 0..255 {
        log_table[x as usize] = i as u8;
        antilog_table[i] = x;
        let mut x_u16 = (x as u16) << 1;

        // Vérifier si le bit 9 (valeur 256) est activé
        if x_u16 > 255 {
            // Réduire par le polynôme irréductible (on applique le XOR sur les 8 bits inférieurs)
            x_u16 ^= primitive_polynomial as u16;
        }

        // Ramener à la taille de u8 après la réduction
        x = x_u16 as u8;
    }
    antilog_table[255] = antilog_table[0]; // Boucle pour utiliser α^255 = α^0
                                           // println!("antilog_table: {antilog_table:?} \n\n");
                                           // println!("log_table: {log_table:?} \n\n");
    (log_table, antilog_table)
}

/// Multiplie deux éléments dans GF(256) en utilisant les tables de log et antilog
fn multiply(a: u8, b: u8, log_table: &Vec<u8>, antilog_table: &Vec<u8>) -> u8 {
    if a == 0 || b == 0 {
        0
    } else {
        let log_a = log_table[a as usize] as usize;
        let log_b = log_table[b as usize] as usize;
        let log_result = (log_a + log_b) % 255; // On reste dans GF(256) (mod 255)
        antilog_table[log_result]
    }
}

pub(crate) fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiplie deux polynômes représentés par des vecteurs dans GF(256).
/// Le résultat est un nouveau polynôme également représenté sous forme de vecteur.
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
            result[i + j] = add(
                result[i + j],
                multiply(p_coef, q_coef, &log_table, &antilog_table),
            );
        }
    }

    result
}

pub(crate) fn get_generator(exp: u32) -> Vec<u8> {
    let (log_table, antilog_table) = init_tables();
    let mut coeff = 1;
    let mut result: Vec<u8> = vec![1, 1];
    if exp == 1 {
        return result;
    }
    for _ in 1..exp {
        coeff = multiply(2, coeff, &log_table, &antilog_table);
        result = multiply_polynomials(&result, &vec![coeff, 1], &log_table, &antilog_table);
    }
    result
}

pub(crate) fn adjust_then_xor(
    message: Vec<u8>,
    generator: &Vec<u8>,
    log_table: &Vec<u8>,
    antilog_table: &Vec<u8>,
) -> Vec<u8> {
    println!("message: {:?} - {}", message, message.len());
    println!("generator: {:?} - {}", generator, generator.len());

    // generator must be multiplies with main coeff of message
    let generator = multiply_polynomials(
        &vec![message.last().unwrap().clone()],
        &generator,
        &log_table,
        &antilog_table,
    );

    println!("adusted_generator: {:?} - {}", generator, generator.len());

    let mut result: Vec<u8> = vec![0; generator.len()];

    // Determing result of XOR between message and generator
    for i in 0..message.len() {
        result[i] = generator[i] ^ message[i];
    }

    println!("result: {:?} - {}\n", result, result.len());

    result
}

pub(crate) fn reed_solomon(mut message: Vec<u8>) -> Vec<u8> {
    println!("********* Appying reed_solomons *********\n\n");

    println!("original_message: {message:?}");

    // Initializing log/antilog tables and parameters to use
    let (log_table, antilog_table) = init_tables();
    let number_divisions = message.len(); // Number of iterations to get error correction codewords, also represents padding length for generator
    let number_ecc = 10; // Number of ecc we want to get, TODO: determine dynamically
    let mut generator = get_generator(number_ecc); // Generator has shape (x+1)(x+2)...(x+2^n)
    let message_padding_length = generator.len(); // Used to determine padding for generator

    println!("original_generator: {generator:?} \n");

    // Adding padding to both message and generator so that they are the same length
    for _ in 0..number_divisions - 1 {
        generator.insert(0, 0);
    }
    for _ in 0..message_padding_length - 1 {
        message.insert(0, 0);
    }

    println!("modified_message: {message:?}");
    println!("modified generator: {generator:?} \n");

    // First iteration
    println!("Iter: 1");
    message = adjust_then_xor(message, &generator, &log_table, &antilog_table);

    // Other iterations
    for i in 1..number_divisions {
        println!("Iter: {}", i + 1);
        message.pop();
        generator.remove(0);
        message = adjust_then_xor(message, &generator, &log_table, &antilog_table);
    }
    message.pop();
    message
}
