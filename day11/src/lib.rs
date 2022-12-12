use std::collections::VecDeque;
use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;
use num::integer::lcm;

type MonkeyIndex = usize;
type WorryLevel = u64;

struct Monkey {
    items: VecDeque<WorryLevel>,
    operation_fn: Box<dyn FnMut(WorryLevel) -> WorryLevel>,
    test_divisor: u64,
    test_outcome: (MonkeyIndex, MonkeyIndex),
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>,
    inspections: Vec<u64>,
    worry_divisor: u64,
    lowest_common_multiple: u64,
}

impl Monkey {
    fn new(
        items: VecDeque<WorryLevel>,
        operation_fn: Box<dyn FnMut(u64) -> u64>,
        test_divisor: u64,
        test_outcome: (MonkeyIndex, MonkeyIndex),
    ) -> Monkey {
        Monkey {
            items,
            operation_fn,
            test_divisor,
            test_outcome,
        }
    }

    fn inspect(&mut self, worry_divisor: u64) -> Option<(MonkeyIndex, WorryLevel)> {
        let worry_level = self.items.pop_front()?;
        let new_worry_level = (self.operation_fn)(worry_level) / worry_divisor;
        let index = if new_worry_level % self.test_divisor == 0 {
            self.test_outcome.0
        } else {
            self.test_outcome.1
        };
        Some((index, new_worry_level))
    }

    fn add_item(&mut self, worry_level: WorryLevel) {
        self.items.push_back(worry_level);
    }
}

impl KeepAwayGame {
    fn new(monkeys: Vec<Monkey>, worry_divisor: u64) -> KeepAwayGame {
        let inspections = vec![0; monkeys.len()];
        let lowest_common_multiple = monkeys
            .iter()
            .fold(1, |cur, monkey| lcm(cur, monkey.test_divisor));
        KeepAwayGame {
            monkeys,
            inspections,
            worry_divisor,
            lowest_common_multiple,
        }
    }

    fn run_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((index, mut worry_level)) = self.monkeys[i].inspect(self.worry_divisor) {
                worry_level %= self.lowest_common_multiple;
                self.monkeys[index].add_item(worry_level);
                self.inspections[i] += 1;
            }
        }
    }

    fn run_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.run_round();
        }
    }
}

fn parse(filename: &str) -> Vec<Monkey> {
    fn parse_monkey_number(input: &str) -> IResult<&str, usize> {
        map(
            delimited(tag("Monkey "), digit1, pair(tag(":"), multispace1)),
            |s: &str| s.parse().unwrap(),
        )(input)
    }

    fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
        map(
            delimited(
                tag("Starting items: "),
                separated_list0(tag(", "), digit1),
                multispace1,
            ),
            |s: Vec<&str>| s.iter().map(|s| s.parse().unwrap()).collect(),
        )(input)
    }

    fn parse_operation(input: &str) -> IResult<&str, Box<dyn FnMut(u64) -> u64>> {
        fn parse_sign(input: &str) -> IResult<&str, char> {
            one_of("+*")(input)
        }

        fn parse_number(input: &str) -> IResult<&str, WorryLevel> {
            map_res(digit1, |n: &str| n.parse())(input)
        }

        fn parse_old(input: &str) -> IResult<&str, &str> {
            tag("old")(input)
        }

        let (input, _) = tag("Operation: new = old ")(input)?;
        let (input, sign) = terminated(parse_sign, space0)(input)?;
        let (input, num) = alt((map(parse_number, Some), map(parse_old, |_| None)))(input)?;
        let (input, _) = multispace1(input)?;
        Ok((
            input,
            match sign {
                '+' => Box::new(move |old: WorryLevel| old + num.unwrap_or(old)),
                '*' => Box::new(move |old: WorryLevel| old * num.unwrap_or(old)),
                _ => unreachable!(),
            },
        ))
    }

    fn parse_test(input: &str) -> IResult<&str, (u64, (MonkeyIndex, MonkeyIndex))> {
        let (input, divisor) = delimited(
            tag("Test: divisible by "),
            map_res(digit1, |s: &str| s.parse()),
            multispace1,
        )(input)?;
        let (input, true_index) = delimited(
            tag("If true: throw to monkey "),
            map_res(digit1, |s: &str| s.parse()),
            multispace1,
        )(input)?;
        let (input, false_index) = delimited(
            tag("If false: throw to monkey "),
            map_res(digit1, |s: &str| s.parse()),
            multispace1,
        )(input)?;
        Ok((input, (divisor, (true_index, false_index))))
    }

    fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                parse_monkey_number,
                parse_starting_items,
                parse_operation,
                parse_test,
            )),
            |(_, items, operation_fn, (test_divisor, test_outcome))| {
                Monkey::new(items, operation_fn, test_divisor, test_outcome)
            },
        )(input)
    }

    fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
        many1(parse_monkey)(input)
    }

    let input = fs::read_to_string(filename).unwrap();
    let (_, instructions) = all_consuming(parse_monkeys)(input.as_str())
        .finish()
        .unwrap();
    instructions
}

pub fn monkey_business_after_rounds(filename: &str, rounds: usize, worry_divisor: u64) -> u64 {
    let mut game = KeepAwayGame::new(parse(filename), worry_divisor);
    game.run_rounds(rounds);
    let mut inspections = game.inspections.clone();
    inspections.sort();
    return inspections.iter().rev().take(2).product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let monkey_business = monkey_business_after_rounds("assets/example.txt", 20, 3);
        assert_eq!(monkey_business, 10605)
    }

    #[test]
    fn part2() {
        let monkey_business = monkey_business_after_rounds("assets/example.txt", 10000, 1);
        assert_eq!(monkey_business, 2713310158)
    }
}
