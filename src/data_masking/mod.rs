use data_masking::{
    eigth_mask, fifth_mask, first_mask, fourth_mask, second_mask, seventh_mask, sixth_mask,
    third_mask,
};
use evaluate_masking::evaluate_masking;

use crate::module_placement::display;

pub(crate) mod data_masking;
pub(crate) mod evaluate_masking;

#[cfg(test)]
pub(crate) mod tests;
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

    println!("Five mask \n");
    let fifth_mask = five_mask(data.clone(), n);
    display(&five_mask, n);
    print!("Seventh Score is: ");
    let five_score = evaluate_masking(five_mask);

    println!("Sixth mask \n");
    let six_mask = sixth_mask(data.clone(), n);
    display(&six_mask, n);
    print!("Seventh Score is: ");
    let six_score = evaluate_masking(six_mask);

    println!("Seventh mask \n");
    let seven_mask = seventh_mask(data.clone(), n);
    display(&seven_mask, n);
    print!("Seventh Score is: ");
    let seven_score = evaluate_masking(seven_mask);

    println!("Eigth mask \n");
    let eigth_mask = eigth_mask(data.clone(), n);
    display(&eigth_mask, n);
    print!("Eight Score is: ");
    let eigth_score = evaluate_masking(eigth_mask);
}
