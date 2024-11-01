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

pub(crate) fn module_placement(data: Vec<u8>) {
    // data is the result from previous step (here: the adding of reed-soloms error correction codewords)
    // We define, which is simply the len of qrcode, here
    let n = 21;

    let mut bits: Vec<u8> = Vec::new();
    for value in data.iter() {
        let mut v = *value; // Create a mutable copy to shift
        for _ in 0..8 {
            bits.push((v & 128) >> 7);
            v <<= 1; // Shift left to prepare for the next bit
        }
    }
    println!("Len: {}", bits.len());
    let mut single_line_bytes = String::new();

    for bite in data {
        single_line_bytes.push_str(&format!("{bite:08b}"));
    }

    println!("Bits on a single line: {single_line_bytes}");
    let mut test = String::new();
    for i in (0..4).rev() {
        for j in 0..(n - 9) {
            test.push_str(&bits[2 * i * (n - 9) + j].to_string());
            test.push_str(&bits[(2 * i + 1) * (n - 9) + j].to_string());
        }
    }
    println!("{test}");
    println!("Vector bits: {:?}", bits); // Prints each bit, start

    let mut bits = Chain::new(bits);

    // First we define the size of our vector, here we have a 21 x 21 for a 1-version with error correction L
    let mut result: Vec<u8> = vec![0; n * n];

    // Then we place function paterns: here only finder, timinig patterns, seperators and dark module

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

    // ===== Placing data inside matrix =====

    // First four columns (space under the right finder pattern)
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
    // Filling space between finder patterns and avoiding horizontal timing pattern
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

    // Filling space between vertical timing and rest of data
    for j in 0..4 {
        result[(n - 9 - j) * n + 8] = bits.next().unwrap();
        result[(n - 9 - j) * n + 7] = bits.next().unwrap();
    }

    // Filling space between the two left finder patterns-
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
}

// We define a display function for debug purposes and encoding visualiton
fn display(list: &Vec<u8>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            // line.push_str(&format!(" {} ", &list[i * n + j].to_string()));
            print!("{:4}", &list[i * n + j].to_string())
        }
        println!("\n");
    }
}

// // We define a function to access an element in vector at given position (x,y) in matrix, considering indexing begins at 0, n is still the matrix dimension
// fn get_element(list: Vec<u8>, x: u8, y: u8, n: u8) {
//     list.get(x*n + y)
// }
