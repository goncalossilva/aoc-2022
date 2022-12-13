use day13::{multiply_divider_packet_indices, sum_packet_indices_in_right_order};

fn main() {
    println!(
        "Part one: {}",
        sum_packet_indices_in_right_order("day13/assets/input.txt")
    );

    println!(
        "Part two: {}",
        multiply_divider_packet_indices("day13/assets/input.txt")
    );
}
