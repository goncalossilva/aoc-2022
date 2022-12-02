use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

pub fn total_score_selected(filename: String) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let guide: Vec<(&str, &str)> = input
        .split_terminator('\n')
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .collect();

    // A = X = Rock
    // B = Y = Paper
    // C = Z = Scissors
    let shape_scores = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let result_scores = HashMap::from([("win", 6), ("draw", 3), ("lose", 0)]);

    return guide.iter().fold(0, |total, &(opponent, selected)| {
        let result = match (opponent, selected) {
            ("A", "Y") | ("B", "Z") | ("C", "X") => "win",
            ("A", "X") | ("B", "Y") | ("C", "Z") => "draw",
            _ => "lose",
        };
        total + shape_scores[selected] + result_scores[result]
    });
}

pub fn total_score_result(filename: String) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let guide: Vec<(&str, &str)> = input
        .split_terminator('\n')
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .collect();

    // A = Rock
    // B = Paper
    // C = Scissors
    // X = Lose
    // Y = Draw
    // Z = Win
    let shape_scores = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let result_scores = HashMap::from([("Z", 6), ("Y", 3), ("X", 0)]);

    return guide.iter().fold(0, |total, &(opponent, result)| {
        let selected = match (opponent, result) {
            ("A", "Y") | ("B", "X") | ("C", "Z") => "A",
            ("B", "Y") | ("C", "X") | ("A", "Z") => "B",
            _ => "C",
        };
        total + shape_scores[selected] + result_scores[result]
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = total_score_selected("assets/example.txt".to_string());
        assert_eq!(result, 15);
    }

    #[test]
    fn part2() {
        let result = total_score_result("assets/example.txt".to_string());
        assert_eq!(result, 12);
    }
}
