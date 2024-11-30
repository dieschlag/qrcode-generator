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

// fn horizontal_penalty(data: &Vec<u8>, n: usize) -> u32 {
//     return 0;
// }

fn horizontal_penalty(data: &Vec<u8>, n: usize) -> u32 {
    let mut penalty = 0;

    // Parcourir chaque ligne de la matrice
    for row in 0..n {
        let mut col = 0;

        while col < n {
            let mut consecutive_count = 1; // Compteur de consécutifs

            // Compter les éléments consécutifs
            while col + 1 < n && data[row * n + col] == data[row * n + col + 1] {
                consecutive_count += 1;
                col += 1;
            }

            // Appliquer la pénalité si une bande de 5 ou plus est trouvée
            if consecutive_count >= 5 {
                penalty += 3; // Pénalité de base pour 5 éléments
                penalty += (consecutive_count - 5) as u32; // Pénalité pour les éléments supplémentaires
            }

            // Passer à l'élément suivant après la bande
            col += 1;
        }
    }

    penalty
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
