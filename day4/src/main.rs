use day4::{count_subranges, fully_contains, partially_contains};

fn main() {
    println!(
        "Part one: {}",
        count_subranges("day4/assets/input.txt".to_string(), fully_contains)
    );

    println!(
        "Part two: {}",
        count_subranges("day4/assets/input.txt".to_string(), partially_contains)
    );
}
