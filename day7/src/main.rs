use day7::{smallest_directory_to_free_up, sum_total_size_of_directories_up_to};

fn main() {
    println!(
        "Part one: {}",
        sum_total_size_of_directories_up_to("day7/assets/input.txt".to_string(), 100000)
    );

    println!(
        "Part two: {}",
        smallest_directory_to_free_up("day7/assets/input.txt".to_string(), 70000000, 30000000)
    );
}
