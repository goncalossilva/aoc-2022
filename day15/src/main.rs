use day15::{count_positions_without_beacon_on_row, tuning_frequency_of_distress_signal};

fn main() {
    println!(
        "Part one: {}",
        count_positions_without_beacon_on_row("day15/assets/input.txt", 2000000)
    );

    println!(
        "Part two: {}",
        tuning_frequency_of_distress_signal("day15/assets/input.txt", 0, 4000000).unwrap()
    );
}
