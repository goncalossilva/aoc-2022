use day2::{total_score_result, total_score_selected};

fn main() {
    println!(
        "Part one: {}",
        total_score_selected("day2/assets/input.txt".to_string())
    );

    println!(
        "Part two: {}",
        total_score_result("day2/assets/input.txt".to_string())
    );
}
