use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, opt};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::{Finish, IResult};
use std::fs;

#[derive(Debug)]
struct Noop;

#[derive(Debug)]
struct Addx(isize);

#[derive(Debug)]
enum Instruction {
    Noop(Noop),
    Addx(Addx),
}

struct Program {
    instructions: Vec<Instruction>,
    x: Vec<isize>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            x: Vec::new(),
        }
    }

    fn run(&mut self) {
        let mut next = 1;
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Noop(_) => self.x.push(next),
                Instruction::Addx(addx) => {
                    self.x.push(next);
                    self.x.push(next);
                    next += addx.0;
                }
            }
        }
    }
}

fn parse_instructions(filename: &str) -> Vec<Instruction> {
    fn parse_noop(i: &str) -> IResult<&str, Noop> {
        map(tag("noop"), |_| Noop)(i)
    }

    fn parse_addx(str: &str) -> IResult<&str, Addx> {
        map(preceded(tag("addx "), parse_value), Addx)(str)
    }

    fn parse_value(str: &str) -> IResult<&str, isize> {
        let (str, sign) = opt(one_of("+-"))(str)?;
        let digit = map_res(digit1, |s: &str| s.parse::<isize>())(str);
        match sign {
            Some('-') => digit.map(|(str, value)| (str, -value)),
            _ => digit,
        }
    }

    fn parse_instruction(str: &str) -> IResult<&str, Instruction> {
        alt((
            map(parse_noop, Instruction::Noop),
            map(parse_addx, Instruction::Addx),
        ))(str)
    }

    fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        terminated(separated_list1(newline, parse_instruction), newline)(input)
    }

    let input = fs::read_to_string(filename).unwrap();
    let (_, instructions) = all_consuming(parse_instructions)(input.as_str())
        .finish()
        .unwrap();
    instructions
}

pub fn sum_signal_strengths(filename: &str, cycles: Vec<usize>) -> isize {
    let mut program = Program::new(parse_instructions(filename));
    program.run();
    program
        .x
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .filter(|(i, _)| cycles.contains(i))
        .map(|(i, x)| i as isize * x)
        .sum()
}

pub fn draw_crt(filename: &str, line_width: usize) -> String {
    let mut program = Program::new(parse_instructions(filename));
    program.run();
    let x = program.x;
    x.iter()
        .enumerate()
        .fold("".to_string(), |mut acc, (i, _)| {
            let max_row = line_width - 1;
            let pixel = (i % line_width) as isize;
            let sprite = x[i] - 1..=x[i] + 1;
            acc += if sprite.contains(&pixel) { "#" } else { "." };
            if i % line_width == max_row {
                acc += "\n"
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sum = sum_signal_strengths("assets/example.txt", vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(sum, 13140)
    }

    #[test]
    fn part2() {
        let crt = draw_crt("assets/example.txt", 40);
        assert_eq!(
            crt.trim(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
            "
            .trim()
        )
    }
}
