pub(crate) fn data_masking(data: Vec<u8>) {
    let n = 21;
}

pub(crate) fn evaluate_masking(data: Vec<u8>) {
    let mut penalty = 0;
    let n = 21;

    // --------------------- Line of 5+ ---------------------

    let vertical_penalties = vertical_penalty(&data, n);
    let horizontal_penalties = horizontal_penalty(&data, n);

    penalty = vertical_penalties + horizontal_penalties
}

fn vertical_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    for column in 0..n {
        let mut i = 0;
        let mut length = 0;
        while i + 1 < n {
            while data[column + i * n] == data[column + (i + 1) * n] {
                length += 1;
                i += 1;
            }

            if length >= 5 {
                penalty += 3 + (length - 5)
            }
            length = 0
        }
    }
    return penalty;
}

fn horizontal_penalty(data: &Vec<u8>, n: usize) -> u32 {
    return 0;
}

fn square_penalty(data: &Vec<u8>, n: usize) -> u32 {
    return 0;
}

fn pattern_penalty(data: &Vec<u8>, n: usize) -> u32 {
    return 0;
}

fn ratio_penalty(data: &Vec<u8>, n: usize) -> u32 {
    return 0;
}
