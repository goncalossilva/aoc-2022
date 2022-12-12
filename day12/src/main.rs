use day12::{
    minimum_steps_to_destination_from_best_position, minimum_steps_to_destination_from_start,
};

fn main() {
    println!(
        "Part one: {}",
        minimum_steps_to_destination_from_start("day12/assets/input.txt")
    );

    println!(
        "Part two: {}",
        minimum_steps_to_destination_from_best_position("day12/assets/input.txt")
    );
}
