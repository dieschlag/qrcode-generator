use evaluate_masking::evaluate_masking;
use masking::{
    eigth_mask, five_mask, four_mask, one_mask, seven_mask, six_mask, three_mask, two_mask,
};

use crate::module_placement::display;

pub(crate) mod evaluate_masking;
pub(crate) mod masking;

pub(crate) fn data_masking(data: Vec<u8>) -> (Vec<u8>, u8) {
    let n = 21;
    // =============== Applying first mask ===============
    println!("First mask \n");
    let one_mask = one_mask(data.clone(), n);
    display(&one_mask, n);
    let one_score = evaluate_masking(&one_mask);
    println!("First Score is: {}\n", one_score);

    println!("Second mask \n");
    let two_mask = two_mask(data.clone(), n);
    display(&two_mask, n);
    let two_score = evaluate_masking(&two_mask);
    println!("Second Score is: {}\n", two_score);

    println!("Third mask \n");
    let three_mask = three_mask(data.clone(), n);
    display(&three_mask, n);
    let three_score = evaluate_masking(&three_mask);
    println!("Third Score is: {}\n", three_score);

    println!("Fourth mask \n");
    let four_mask = four_mask(data.clone(), n);
    display(&four_mask, n);
    let four_score = evaluate_masking(&four_mask);
    println!("Fourth Score is: {}\n", four_score);

    println!("Fifth mask \n");
    let five_mask = five_mask(data.clone(), n);
    display(&five_mask, n);
    let five_score = evaluate_masking(&five_mask);
    println!("Fifth Score is: {}\n", five_score);

    println!("Sixth mask \n");
    let six_mask = six_mask(data.clone(), n);
    display(&six_mask, n);
    let six_score = evaluate_masking(&six_mask);
    println!("Sixth Score is: {}\n", six_score);

    println!("Seventh mask \n");
    let seven_mask = seven_mask(data.clone(), n);
    display(&seven_mask, n);
    let seven_score = evaluate_masking(&seven_mask);
    println!("Seventh Score is: {}\n", seven_score);

    println!("Eigth mask \n");
    let eight_mask = eigth_mask(data.clone(), n);
    display(&eight_mask, n);
    let eight_score = evaluate_masking(&eight_mask);
    println!("Eight Score is: {}\n", eight_score);

    let list_score = [
        one_score,
        two_score,
        three_score,
        four_score,
        five_score,
        six_score,
        seven_score,
        eight_score,
    ];

    let (index, best_score) = list_score
        .iter()
        .enumerate()
        .min_by_key(|&(_, &value)| value)
        .unwrap();

    println!("Best score is {}", best_score);

    let list_mask = [
        one_mask, two_mask, three_mask, four_mask, five_mask, six_mask, seven_mask,
    ];

    let best_mask = list_mask.get(index).unwrap();
    println!("Best mask applied is: {}", index);
    display(best_mask, 21);
    ((*best_mask).clone(), index as u8)
}

#[cfg(test)]
pub(crate) mod tests;
