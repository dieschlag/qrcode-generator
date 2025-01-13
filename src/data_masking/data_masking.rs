use crate::module_placement::display;

pub(crate) fn data_masking(data: Vec<u8>) {
    let n = 21;
    println!("test_switch");
    test_switch();
    // =============== Applying first mask ===============

    let first_mask = first_mask(data, n);

    display(&first_mask, n);
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

pub(crate) fn apply_first_mask(bit: &mut u8, row: usize, col: usize) {
    if (row + col) % 2 == 0 {
        switch(bit)
    }
}

// pub(crate) fn apply_first_mask(bit: &mut u8, row: usize, col: usize) {
//     if (row + col) % 2 == 0 {
//         switch(bit)
//     }
// }

// pub(crate) fn apply_first_mask(bit: &mut u8, row: usize, col: usize) {
//     if (row + col) % 2 == 0 {
//         switch(bit)
//     }
// }

// pub(crate) fn apply_first_mask(bit: &mut u8, row: usize, col: usize) {
//     if (row + col) % 2 == 0 {
//         switch(bit)
//     }
// }

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
