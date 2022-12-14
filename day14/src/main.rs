use day14::count_sand;

fn main() {
    println!("Part one: {}", count_sand("day14/assets/input.txt", None));

    println!(
        "Part two: {}",
        count_sand("day14/assets/input.txt", Some(2))
    );
}
