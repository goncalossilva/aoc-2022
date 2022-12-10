use day10::{draw_crt, sum_signal_strengths};

fn main() {
    println!(
        "Part one: {}",
        sum_signal_strengths("day10/assets/input.txt", vec![20, 60, 100, 140, 180, 220])
    );

    println!("Part two:\n{}", draw_crt("day10/assets/input.txt", 40));
}
