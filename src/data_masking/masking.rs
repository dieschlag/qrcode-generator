// Fist mask

pub(crate) fn apply_one_mask(bit: &mut u8, row: usize, col: usize) {
    if (row + col) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn one_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_one_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_one_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_one_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Second mask

pub(crate) fn apply_two_mask(bit: &mut u8, row: usize) {
    if row % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn two_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_two_mask(&mut data[i * n + j], i);
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_two_mask(&mut data[i * n + j], i);
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_two_mask(&mut data[i * n + j], i);
            }
        }
    }
    data
}

// Third mask

pub(crate) fn apply_three_mask(bit: &mut u8, column: usize) {
    if column % 3 == 0 {
        switch(bit)
    }
}

pub(crate) fn three_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_three_mask(&mut data[i * n + j], i)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_three_mask(&mut data[i * n + j], i)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_three_mask(&mut data[i * n + j], i)
            }
        }
    }
    data
}

// Fourth mask

pub(crate) fn apply_four_mask(bit: &mut u8, row: usize, column: usize) {
    if (column + row) % 3 == 0 {
        switch(bit)
    }
}

pub(crate) fn four_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_four_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_four_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_four_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Fifth mask

pub(crate) fn apply_five_mask(bit: &mut u8, row: usize, column: usize) {
    if (row / 2 + column / 3) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn five_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_five_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_five_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_five_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Sixth mask

pub(crate) fn apply_six_mask(bit: &mut u8, row: usize, column: usize) {
    if ((row * column) % 2 + (row * column) % 3) == 0 {
        switch(bit)
    }
}

pub(crate) fn six_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_six_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_six_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_six_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Seventh mask

pub(crate) fn apply_seven_mask(bit: &mut u8, row: usize, column: usize) {
    if ((row * column) % 2 + (row * column) % 3) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn seven_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_seven_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_seven_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_seven_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Seventh mask

pub(crate) fn apply_eigth_mask(bit: &mut u8, row: usize, column: usize) {
    if ((row + column) % 2 + (row * column) % 3) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn eigth_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_eigth_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 6 {
                apply_eigth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 6 {
                apply_eigth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Handlers

pub(crate) fn switch(bit: &mut u8) {
    if *bit == 1 {
        *bit = 0;
    } else {
        *bit = 1;
    }
}
