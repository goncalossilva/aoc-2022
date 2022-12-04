use itertools::Itertools;
use std::fs;
use std::ops::RangeInclusive;

pub fn count_subranges(
    filename: String,
    criteria_fn: fn(RangeInclusive<usize>, RangeInclusive<usize>) -> bool,
) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let pairs = input.lines();

    pairs
        .filter(|pair| {
            let (first, second) = pair
                .split(',')
                .map(|range_str| {
                    let (min, max) = range_str
                        .split('-')
                        .map(|s| s.parse::<usize>().unwrap())
                        .next_tuple::<(usize, usize)>()
                        .unwrap();
                    min..=max
                })
                .collect_tuple::<(_, _)>()
                .unwrap();
            criteria_fn(first, second)
        })
        .count()
}

pub fn fully_contains<T: Ord>(first: RangeInclusive<T>, second: RangeInclusive<T>) -> bool {
    return first.start() <= second.start() && first.end() >= second.end()
        || second.start() <= first.start() && second.end() >= first.end();
}

pub fn partially_contains<T: Ord>(first: RangeInclusive<T>, second: RangeInclusive<T>) -> bool {
    return first.start() <= second.start() && second.start() <= first.end()
        || first.start() <= second.end() && second.end() <= first.end()
        || second.start() <= first.start() && first.start() <= second.end()
        || second.start() <= first.end() && first.end() <= second.end();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = count_subranges("assets/example.txt".to_string(), fully_contains);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let result = count_subranges("assets/example.txt".to_string(), partially_contains);
        assert_eq!(result, 4);
    }
}
