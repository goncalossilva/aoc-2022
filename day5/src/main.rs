use day5::crates_on_top;

fn main() {
    println!(
        "Part one: {}",
        crates_on_top("day5/assets/input.txt".to_string(), false)
    );

    println!(
        "Part two: {}",
        crates_on_top("day5/assets/input.txt".to_string(), true)
    );
}
