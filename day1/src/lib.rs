use std::fs;

pub fn most_calories(filename: String, elf_count: usize) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let mut cals_per_elf: Vec<i32> = input
        .split_terminator("\n\n")
        .map(|elf_cals| {
            elf_cals
                .lines()
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    cals_per_elf.sort();
    cals_per_elf.reverse();

    return cals_per_elf[..elf_count].iter().sum::<i32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = most_calories("assets/example.txt".to_string(), 1);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2() {
        let result = most_calories("assets/example.txt".to_string(), 3);
        assert_eq!(result, 45000);
    }
}
