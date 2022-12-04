use day3::{sum_priorities_in_both_compartments, sum_priorities_in_groups_of_three};

fn main() {
    println!(
        "Part one: {}",
        sum_priorities_in_both_compartments("day3/assets/input.txt".to_string())
    );

    println!(
        "Part two: {}",
        sum_priorities_in_groups_of_three("day3/assets/input.txt".to_string())
    );
}
