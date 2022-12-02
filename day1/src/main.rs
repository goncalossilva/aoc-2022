use day1::most_calories;

fn main() {
    println!(
        "Part one: {}",
        most_calories("day1/assets/input.txt".to_string(), 1)
    );

    println!(
        "Part two: {}",
        most_calories("day1/assets/input.txt".to_string(), 3)
    );
}
