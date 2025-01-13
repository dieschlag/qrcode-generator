use crate::module_placement::display;

pub(crate) fn data_masking(data: Vec<u8>) {
    let n = 21;
    // =============== Applying first mask ===============
    println!("First mask \n");
    let first_mask = first_mask(data.clone(), n);
    display(&first_mask, n);

    println!("Second mask \n");
    let second_mask = second_mask(data.clone(), n);
    display(&second_mask, n);

    println!("Third mask \n");
    let third_mask = third_mask(data.clone(), n);
    display(&third_mask, n);

    println!("Fourth mask \n");
    let fourth_mask = fourth_mask(data.clone(), n);
    display(&fourth_mask, n);

    println!("Fifth mask \n");
    let fifth_mask = fifth_mask(data.clone(), n);
    display(&fifth_mask, n);

    println!("Sixth mask \n");
    let sixth_mask = sixth_mask(data.clone(), n);
    display(&sixth_mask, n);

    println!("Seventh mask \n");
    let seventh_mask = seventh_mask(data.clone(), n);
    display(&seventh_mask, n);

    println!("Eigth mask \n");
    let eigth_mask = eigth_mask(data.clone(), n);
    display(&eigth_mask, n);
}

// Fist mask

pub(crate) fn apply_first_mask(bit: &mut u8, row: usize, col: usize) {
    if (row + col) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn first_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_first_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_first_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_first_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Second mask

pub(crate) fn apply_second_mask(bit: &mut u8, row: usize) {
    if row % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn second_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_second_mask(&mut data[i * n + j], i)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_second_mask(&mut data[i * n + j], i)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_second_mask(&mut data[i * n + j], i)
            }
        }
    }
    data
}

// Third mask

pub(crate) fn apply_third_mask(bit: &mut u8, column: usize) {
    if column % 3 == 0 {
        switch(bit)
    }
}

pub(crate) fn third_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_third_mask(&mut data[i * n + j], i)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_third_mask(&mut data[i * n + j], i)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_third_mask(&mut data[i * n + j], i)
            }
        }
    }
    data
}

// Fourth mask

pub(crate) fn apply_fourth_mask(bit: &mut u8, row: usize, column: usize) {
    if (column + row) % 3 == 0 {
        switch(bit)
    }
}

pub(crate) fn fourth_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_fourth_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_fourth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_fourth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Fifth mask

pub(crate) fn apply_fifth_mask(bit: &mut u8, row: usize, column: usize) {
    if (row / 2 + column / 3) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn fifth_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_fifth_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_fifth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_fifth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Sixth mask

pub(crate) fn apply_sixth_mask(bit: &mut u8, row: usize, column: usize) {
    if ((row * column) % 2 + (row * column) % 3) == 0 {
        switch(bit)
    }
}

pub(crate) fn sixth_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_sixth_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_sixth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_sixth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    data
}

// Seventh mask

pub(crate) fn apply_seventh_mask(bit: &mut u8, row: usize, column: usize) {
    if ((row * column) % 2 + (row * column) % 3) % 2 == 0 {
        switch(bit)
    }
}

pub(crate) fn seventh_mask(mut data: Vec<u8>, n: usize) -> Vec<u8> {
    for i in 9..n {
        for j in (n - 8)..n {
            apply_seventh_mask(&mut data[i * n + j], i, j)
        }
    }
    for i in 0..n {
        for j in 9..(n - 8) {
            if i != 7 {
                apply_seventh_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
                apply_seventh_mask(&mut data[i * n + j], i, j)
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
            if i != 7 {
                apply_eigth_mask(&mut data[i * n + j], i, j)
            }
        }
    }
    for i in 9..(n - 8) {
        for j in 0..9 {
            if j != 7 {
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

pub(crate) fn test_switch() {
    let mut data = vec![0, 0, 1, 1, 0, 0];
    for bit in &mut data {
        switch(bit);
    }
    println!("{:?}", data);
}

// #[cfg(test)]
// #[test]
// fn test_first_mask() {
//     let data: Vec<u8> = vec![
//         2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//         0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2,
//         2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//         2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2,
//         2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//         2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0,
//         0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2,
//         2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2,
//         2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     ];
//     let first_mask = first_mask(data, 21);
//     assert_eq!(1, 1)
// }
