#[cfg(test)]
mod tests {
    use crate::data_masking::evaluate_masking::{horizontal_penalty, vertical_penalty};

    #[test]
    fn test_horizontal_penalty() {
        let data = vec![
            1, 1, 1, 1, 1, 1, // penalty = 4
            1, 0, 0, 0, 0, 0, // penalty = 3
            1, 1, 1, 1, 0, 0, // penalty = 0
            1, 0, 0, 0, 1, 0, // penalty = 0
            1, 0, 0, 0, 1, 0, // penalty = 0
            1, 0, 0, 0, 1, 0, // penalty = 0
        ]; // n = 6
        let n = 6;
        assert_eq!(horizontal_penalty(&data, n), 7); // Total: 4 (row 1) + 3 (row 3) = 7
    }

    #[test]
    fn test_vertical_penalty() {
        let data = vec![
            1, 0, 0, 1, 0, 1, // column 1: penalty = 4
            1, 0, 0, 0, 1, 1, // column 2: penalty = 0
            1, 1, 0, 0, 1, 1, // column 3: penalty = 0
            1, 0, 1, 0, 1, 1, // column 4: penalty = 3
            1, 0, 0, 0, 1, 1, // column 5: penalty = 0
            1, 0, 0, 0, 0, 0, // column 6: penalty = 3
        ];
        let n = 6;
        assert_eq!(vertical_penalty(&data, n), 10); // Total: 10
    }
}
