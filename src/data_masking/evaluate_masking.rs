pub(crate) fn evaluate_masking(data: Vec<u8>) -> u32 {
    let mut penalty = 0;
    let n = 21;

    let vertical_penalties = vertical_penalty(&data, n);
    let horizontal_penalties = horizontal_penalty(&data, n);

    let square_penalties = square_penalty(&data, n);

    let ratio_penalties = ratio_penalty(&data, n);

    vertical_penalties + horizontal_penalties + ratio_penalties + square_penalties
}

pub(crate) fn vertical_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    for col in 0..n {
        let mut row = 0;

        while row < n {
            let mut consecutive_count = 1;

            while row + 1 < n && data[row * n + col] == data[(row + 1) * n + col] {
                consecutive_count += 1;
                row += 1;
            }

            if consecutive_count >= 5 {
                penalty += 3;
                penalty += (consecutive_count - 5) as u32;
            }

            row += 1;
        }
    }

    penalty
}

pub(crate) fn horizontal_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    for row in 0..n {
        let mut col = 0;

        while col < n {
            let mut consecutive_count = 1;
            while col + 1 < n && data[row * n + col] == data[row * n + col + 1] {
                consecutive_count += 1;
                col += 1;
            }

            if consecutive_count >= 5 {
                penalty += 3;
                penalty += (consecutive_count - 5) as u32;
            }

            col += 1;
        }
    }

    penalty
}

pub fn square_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    // Loop through each possible top-left corner of a 2x2 square
    for row in 0..(n - 1) {
        for col in 0..(n - 1) {
            // Compute the indices of the four elements in the 2x2 square
            let top_left = row * n + col;
            let top_right = row * n + col + 1;
            let bottom_left = (row + 1) * n + col;
            let bottom_right = (row + 1) * n + col + 1;

            // Check if all elements in the square are the same
            if data[top_left] == data[top_right]
                && data[top_left] == data[bottom_left]
                && data[top_left] == data[bottom_right]
            {
                penalty += 3;
            }
        }
    }

    penalty
}

pub(crate) fn pattern_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    let pattern1 = vec![1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0];
    let pattern2 = vec![0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1];

    fn matches_pattern(slice: &[u8], pattern: &[u8]) -> bool {
        slice.len() >= pattern.len() && slice.windows(pattern.len()).any(|window| window == pattern)
    }

    for row in 0..n {
        let start = row * n;
        let end = start + n;
        let row_data = &data[start..end];
        if matches_pattern(row_data, &pattern1) || matches_pattern(row_data, &pattern2) {
            penalty += 40;
        }
    }

    for col in 0..n {
        let mut col_data = Vec::new();
        for row in 0..n {
            col_data.push(data[row * n + col]);
        }
        if matches_pattern(&col_data, &pattern1) || matches_pattern(&col_data, &pattern2) {
            penalty += 40;
        }
    }

    penalty
}

pub fn ratio_penalty(data: &Vec<u8>, n: usize) -> u32 {
    // Step 1: Calculate the percentage of black modules
    let total_modules = n * n;
    let black_count = data.iter().filter(|&&value| value == 1).count();
    let percentage_black = (black_count as f64 / total_modules as f64) * 100.0;

    let closest_multiple_of_5 = (percentage_black / 5.0).floor() * 5.0;

    let abs_original = (closest_multiple_of_5 + 5.0 - 50.0).abs();
    let abs_closest = (closest_multiple_of_5 - 50.0).abs();

    let distance_original = abs_original / 5.0;
    let distance_closest = abs_closest / 5.0;
    let smallest_distance = distance_original.min(distance_closest);

    let penalty = (smallest_distance * 10.0).round() as u32;

    penalty
}
