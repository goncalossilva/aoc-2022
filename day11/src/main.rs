use day11::monkey_business_after_rounds;

fn main() {
    println!(
        "Part one: {}",
        monkey_business_after_rounds("day11/assets/input.txt", 20, 3)
    );

    println!(
        "Part two: {}",
        monkey_business_after_rounds("day11/assets/input.txt", 10000, 1)
    );
}
