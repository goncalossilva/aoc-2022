use itertools::Itertools;
use std::fs;

pub fn sum_priorities_in_both_compartments(filename: String) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let rucksacks = input.lines();

    rucksacks.fold(0, |acc, rucksack| {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        acc + priority(first.chars().find(|c| second.chars().contains(c)).unwrap())
    })
}

pub fn sum_priorities_in_groups_of_three(filename: String) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let rucksacks = input.lines();

    rucksacks.chunks(3).into_iter().fold(0, |acc, mut chunk| {
        let first = chunk.next().unwrap();
        let rest = chunk.collect_vec();
        acc + priority(
            first
                .chars()
                .find(|c| rest.iter().all(|r| r.contains(*c)))
                .unwrap(),
        )
    })
}

fn priority(item_type: char) -> i32 {
    if item_type.is_lowercase() {
        (item_type as u8 - b'a' + 1) as i32
    } else {
        (item_type as u8 - b'A' + 27) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = sum_priorities_in_both_compartments("assets/example.txt".to_string());
        assert_eq!(result, 157);
    }

    #[test]
    fn part2() {
        let result = sum_priorities_in_groups_of_three("assets/example.txt".to_string());
        assert_eq!(result, 70);
    }
}
