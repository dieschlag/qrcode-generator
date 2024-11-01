use polynomials::{init_tables, multiply, multiply_polynomials};

pub(crate) mod polynomials;

pub(crate) fn reed_solomon(mut message: Vec<u8>) -> Vec<u8> {
    println!("**************************** Appying reed_solomons ****************************\n\n");
    println!("Received data: {message:?}");

    // ======================================= Initialization =======================================

    // Initializing log/antilog tables and parameters to use
    let (log_table, antilog_table) = init_tables();

    println!("Generating log/antilog tables:    OK");

    // Parameters for desired qrcode
    let number_divisions = message.len(); // Number of iterations to get error correction codewords, also represents padding length for generator
    let number_ecc = 10; // Number of ecc we want to get, TODO: determine dynamically
    let mut generator = get_generator(number_ecc); // Generator has shape (x+1)(x+2)...(x+2^n) where is the number of ecc we want
    let message_padding_length = generator.len(); // Used to determine padding for message

    println!("Generator has order: {number_ecc}");
    println!("Generating generator:    OK");

    // Adding padding to both message and generator so that they are the same length
    for _ in 0..number_divisions - 1 {
        generator.push(0);
    }
    for _ in 0..message_padding_length - 1 {
        message.push(0);
    }

    println!("modified_message: {message:?}");
    println!("modified generator: {generator:?} \n");

    // =======================================

    // Series of polynomial divisions: first we multiply generator by the lead coeff of message, then we xor the two polynomials term by term
    // Main term of result is 0, we remove it and shorten generator by a power before next iteration
    for _ in 0..number_divisions {
        message = adjust_then_xor(message, &generator, &log_table, &antilog_table);
        message.remove(0);
        generator.pop();
    }

    // Returning message
    message
}

// ===================================================== Handlers =====================================================

// Function to generate generator, it is of the shape (x-2)(x-4)....(x-2^exp) where it is the given order of the generator
pub(crate) fn get_generator(exp: u32) -> Vec<u8> {
    let (log_table, antilog_table) = init_tables();
    let mut coeff = 1;
    let mut result: Vec<u8> = vec![1, 1];
    if exp == 1 {
        return result;
    }
    for _ in 1..exp {
        coeff = multiply(2, coeff, &log_table, &antilog_table);
        result = multiply_polynomials(&result, &vec![1, coeff], &log_table, &antilog_table);
    }
    result
}

// Multiplies generator by main coeff of message, then XOR message and generator term by term
pub(crate) fn adjust_then_xor(
    message: Vec<u8>,
    generator: &Vec<u8>,
    log_table: &Vec<u8>,
    antilog_table: &Vec<u8>,
) -> Vec<u8> {
    // generator must be multiplies with main coeff of message
    let generator = multiply_polynomials(&vec![message[0]], &generator, &log_table, &antilog_table);

    let mut result: Vec<u8> = vec![0; generator.len()];

    // Determing result of XOR between message and generator
    for i in 0..message.len() {
        result[i] = generator[i] ^ message[i];
    }

    result
}
