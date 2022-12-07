use day6::index_of_marker;

fn main() {
    println!(
        "Part one: {}",
        index_of_marker("day6/assets/input.txt".to_string(), 4).unwrap()
    );

    println!(
        "Part two: {}",
        index_of_marker("day6/assets/input.txt".to_string(), 14).unwrap()
    );
}
