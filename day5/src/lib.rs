use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::vec::IntoIter;

struct Stacks(HashMap<usize, Stack>);

#[derive(Debug)]
struct Stack {
    line_index: usize,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Stacks {
    fn new(lines: Vec<String>) -> Self {
        let mut stacks_lines = lines.iter().rev();

        let mut stacks: HashMap<usize, Stack> = stacks_lines
            .next()
            .unwrap()
            .match_indices(|c| ('1'..='9').contains(&c))
            .map(|(i, n)| {
                let number = n.parse::<usize>().unwrap();
                let stack = Stack {
                    line_index: i,
                    crates: Vec::new(),
                };
                (number, stack)
            })
            .into_iter()
            .collect();

        stacks_lines.for_each(|stack_line| {
            stacks.iter_mut().for_each(|(_, stack)| {
                if let Some(c) = stack_line.chars().nth(stack.line_index) {
                    if !c.is_whitespace() {
                        stack.crates.push(c)
                    }
                }
            });
        });

        Stacks(stacks)
    }

    fn numbers(self) -> IntoIter<(usize, Stack)> {
        self.0.into_iter().sorted_by_key(|el| el.0)
    }
}

impl Move {
    fn new(line: String) -> Self {
        let mut tokens = line.split_whitespace();
        tokens.next(); // Skip "move".
        let count: usize = tokens.next().unwrap().parse().unwrap();
        tokens.next(); // Skip "from".
        let from: usize = tokens.next().unwrap().parse().unwrap();
        tokens.next(); // Skip "to".
        let to: usize = tokens.next().unwrap().parse().unwrap();

        Move { count, from, to }
    }

    fn apply(&mut self, stacks: &mut Stacks, move_multiple: bool) {
        let from = stacks.0.get_mut(&self.from).unwrap();
        let to_move = from.crates.split_off(from.crates.len() - self.count);
        let to = stacks.0.get_mut(&self.to).unwrap();
        if move_multiple {
            to.crates.extend(to_move.iter());
        } else {
            to.crates.extend(to_move.iter().rev());
        }
    }
}

pub fn crates_on_top(filename: String, move_multiple: bool) -> String {
    let input = fs::read_to_string(filename).unwrap();
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    // Parse stacks.
    let mut stacks = Stacks::new(stacks_input.lines().map(|l| l.to_string()).collect());

    // Parse moves.
    let moves = moves_input
        .lines()
        .map(|line| Move::new(line.to_string()))
        .collect::<Vec<Move>>();

    // Apply moves.
    for mut mov in moves {
        mov.apply(&mut stacks, move_multiple);
    }

    stacks.numbers().fold(String::new(), |acc, (_, stack)| {
        acc + stack.crates.last().unwrap().to_string().as_str()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = crates_on_top("assets/example.txt".to_string(), false);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2() {
        let result = crates_on_top("assets/example.txt".to_string(), true);
        assert_eq!(result, "MCD");
    }
}
