/// Goal is to get a vector of 1 and 0 matching visual distribution of bits on our QR code before data masking
/// We will use a 1-D vector to represent this distribution
/// This 1-D vector will be used to encode matrix, considering each p elements, we have a new line where n is the number of columns we would have in our matric
/// To note: with QR codes, we use square matrixes so the n is unamiguous between columns and lines

/// Data to encode in QR Code is represented as a Chain, which is a wrapper over a classic vector, with a tracker of current index to travel along data;
pub(crate) struct Chain {
    pub(crate) data: Vec<u8>,
    pub(crate) index: usize,
}

impl Chain {
    pub(crate) fn new(data: Vec<u8>) -> Chain {
        Chain { data, index: 0 }
    }
    pub(crate) fn next(&mut self) -> Option<u8> {
        let result = self.data.get(self.index).copied();
        self.index += 1;
        result
    }
}

/// Third step in QR Code generation before masking data.
/// Module placement uses a 1-D vector to represent data. This vector represents a square matrix of size n, with a new line every n elements in the vector.
/// The number 1 corresponds to black modules and 0 to white modules.
pub(crate) fn module_placement(data: Vec<u8>) -> Vec<u8> {
    //

    println!("**************************** Module Placement ****************************\n");

    // ======================================= Initialization =======================================

    // We define, which is simply the len of qrcode, TODO: make it dynamical
    let n = 21;
    println!("Len of QR Code is: {}\n", n);

    // Vector containing each bit of the bit stream composed of encoded message + error correction codewords
    let mut bits: Vec<u8> = Vec::new();
    for value in data.iter() {
        let mut v = *value; // Creates a mutable copy to shift
        for _ in 0..8 {
            bits.push((v & 128) >> 7); // AND with 128 to extract dominant bit
            v <<= 1; // Shift left to prepare for the next bit
        }
    }

    // Initiate Chain instance
    let mut bits = Chain::new(bits);

    // Initiate result vector
    let mut result: Vec<u8> = vec![0; n * n];

    // ======================================= Patterns placement =======================================

    // ===== Finder paterns =====

    //Horizontal
    for i in 0..7 {
        result[i] = 1;
        result[6 * n + i] = 1;
        result[n - 1 - i] = 1;
        result[6 * n + (n - 1 - i)] = 1;
        result[(n - 1) * n + i] = 1;
        result[(n - 7) * n + i] = 1;
    }

    // Vertical
    for i in 1..6 {
        result[i * n] = 1;
        result[i * n + 6] = 1;
        result[i * n + n - 1] = 1;
        result[i * n + (n - 7)] = 1;
        result[(n - 7 + i) * n] = 1;
        result[(n - 7 + i) * n + 6] = 1;
    }

    // Center
    for i in 0..3 {
        result[(2 * n) + (i + 2)] = 1;
        result[(3 * n) + (i + 2)] = 1;
        result[(4 * n) + (i + 2)] = 1;
        result[(n - 3) * n + (i + 2)] = 1;
        result[(n - 4) * n + (i + 2)] = 1;
        result[(n - 5) * n + (i + 2)] = 1;
        result[(2 * n) + (n - 3 - i)] = 1;
        result[(3 * n) + (n - 3 - i)] = 1;
        result[(4 * n) + (n - 3 - i)] = 1;
    }

    //  ===== Timing Patterns =====

    for i in 7..(n - 7) {
        result[i * n + 6] = ((i + 1) % 2) as u8;
        result[6 * n + i] = ((i + 1) % 2) as u8;
    }

    // ===== Dark Module =====

    result[(n - 7) * n + 8] = 1;

    // ======================================= Data placement =======================================

    // ===== Space under right finder =====
    for i in 0..4 {
        if i % 2 == 0 {
            for j in 0..(n - 9) {
                result[(n - 1 - j) * n + n - 1 - 2 * i] = bits.next().unwrap();
                result[(n - 1 - j) * n + n - 1 - (2 * i + 1)] = bits.next().unwrap();
            }
        } else {
            for j in 9..n {
                result[j * n + n - 1 - (2 * i + 1)] = bits.next().unwrap();

                result[j * n + n - 1 - (2 * i)] = bits.next().unwrap();
            }
        }
    }

    // ===== Space bewtween two finders =====

    for i in 0..2 {
        if i % 2 == 1 {
            for j in 0..n {
                if j != n - 7 {
                    result[(n - 1 - j) * n + n - 9 - 2 * i] = bits.next().unwrap();

                    result[(n - 1 - j) * n + n - 9 - (2 * i + 1)] = bits.next().unwrap();
                }
            }
        } else {
            for j in 0..n {
                if j != 6 {
                    result[j * n + n - 9 - (2 * i + 1)] = bits.next().unwrap();
                    result[j * n + n - 9 - (2 * i)] = bits.next().unwrap();
                }
            }
        }
    }

    // ===== Filling gap before vertical timing =====

    for j in 0..4 {
        result[(n - 9 - j) * n + 8] = bits.next().unwrap();
        result[(n - 9 - j) * n + 7] = bits.next().unwrap();
    }

    // ===== Filling space before the two right finder patterns =====

    for i in 0..3 {
        if i % 2 == 0 {
            for j in 0..4 {
                result[(9 + j) * n + (4 - 2 * i)] = bits.next().unwrap();
                result[(9 + j) * n + (5 - 2 * i)] = bits.next().unwrap();
            }
        } else {
            for j in 0..4 {
                result[(n - 9 - j) * n + (5 - 2 * i)] = bits.next().unwrap();
                result[(n - 9 - j) * n + (4 - 2 * i)] = bits.next().unwrap();
            }
        }
    }

    // Display result for verification
    display(&result, n);
    result
}

/// Used to display the values inside the matrix
pub(crate) fn display(list: &Vec<u8>, n: usize) {
    println!("Result matrix is:\n");
    for i in 0..n {
        for j in 0..n {
            print!("{:2}", &list[i * n + j].to_string())
        }
        println!();
    }
}
