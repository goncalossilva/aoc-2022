use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::newline,
    combinator::map,
    combinator::{all_consuming, opt},
    multi::many1,
    multi::separated_list0,
    sequence::delimited,
    sequence::pair,
    sequence::terminated,
    Finish, IResult,
};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Elem {
    Int(i64),
    List(Vec<Elem>),
}

fn parse(filename: &str) -> Vec<(Elem, Elem)> {
    fn parse_int(input: &str) -> IResult<&str, Elem> {
        map(digit1, |s: &str| Elem::Int(s.parse::<i64>().unwrap()))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Elem> {
        map(
            delimited(tag("["), separated_list0(tag(","), parse_elem), tag("]")),
            Elem::List,
        )(input)
    }

    fn parse_elem(input: &str) -> IResult<&str, Elem> {
        alt((parse_int, parse_list))(input)
    }

    fn parse_pair(input: &str) -> IResult<&str, (Elem, Elem)> {
        pair(
            terminated(parse_elem, newline),
            terminated(parse_elem, newline),
        )(input)
    }

    fn parse_pairs(input: &str) -> IResult<&str, Vec<(Elem, Elem)>> {
        many1(terminated(parse_pair, opt(newline)))(input)
    }

    let input = fs::read_to_string(filename).unwrap();
    let (_, pairs) = all_consuming(parse_pairs)(input.as_str()).finish().unwrap();
    pairs
}

fn is_in_right_order(left: &Elem, right: &Elem) -> Option<bool> {
    match (left, right) {
        (Elem::Int(l), Elem::Int(r)) => match l.cmp(r) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (Elem::List(l), Elem::List(r)) => 'inner: {
            for (l, r) in l.iter().zip(r.iter()) {
                if let Some(bool) = is_in_right_order(l, r) {
                    break 'inner Some(bool);
                }
            }
            break 'inner match l.len().cmp(&r.len()) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            };
        }
        (Elem::Int(l), Elem::List(_)) => is_in_right_order(&Elem::List(vec![Elem::Int(*l)]), right),
        (Elem::List(_), Elem::Int(r)) => is_in_right_order(left, &Elem::List(vec![Elem::Int(*r)])),
    }
}

pub fn sum_packet_indices_in_right_order(filename: &str) -> usize {
    let pairs = parse(filename);
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| is_in_right_order(left, right) == Some(true))
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn multiply_divider_packet_indices(filename: &str) -> usize {
    let pairs = parse(filename);
    let dividers = vec![
        Elem::List(vec![Elem::List(vec![Elem::Int(2)])]),
        Elem::List(vec![Elem::List(vec![Elem::Int(6)])]),
    ];

    // Flatten pairs of packets and add dividers.
    let mut packets = pairs
        .iter()
        .fold(dividers.clone(), |mut acc, (left, right)| {
            acc.push(left.clone());
            acc.push(right.clone());
            acc
        });

    // Sort packets.
    packets.sort_by(|a, b| {
        if let Some(bool) = is_in_right_order(a, b) {
            if bool {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            panic!("Cannot compare {:?} and {:?}", a, b);
        }
    });

    // Get decoder key.
    packets
        .iter()
        .enumerate()
        .filter(|(_, elem)| dividers.contains(elem))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sum = sum_packet_indices_in_right_order("assets/example.txt");
        assert_eq!(sum, 13);
    }

    #[test]
    fn part2() {
        let sum = multiply_divider_packet_indices("assets/example.txt");
        assert_eq!(sum, 140);
    }
}
