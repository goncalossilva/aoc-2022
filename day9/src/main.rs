use day9::count_tail_visited_positions;

fn main() {
    println!(
        "Part one: {}",
        count_tail_visited_positions("day9/assets/input.txt".to_string(), 1)
    );

    println!(
        "Part two: {}",
        count_tail_visited_positions("day9/assets/input.txt".to_string(), 9)
    );
}
