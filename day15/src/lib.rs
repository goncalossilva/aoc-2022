use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;
use std::cmp::max;
use std::cmp::min;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sensor {
    sensor_pos: Coord,
    beacon_pos: Coord,
    distance: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn manhattan_distance(&self, other: &Coord) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialEq<(isize, isize)> for Coord {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Sensor {
    fn new(pos: Coord, beacon: Coord) -> Sensor {
        Sensor {
            distance: pos.manhattan_distance(&beacon),
            sensor_pos: pos,
            beacon_pos: beacon,
        }
    }

    fn within_distance(&self, other: &Coord) -> bool {
        self.sensor_pos.manhattan_distance(other) <= self.distance
    }

    fn skip_distance(&self, other: &Coord) -> isize {
        self.distance - self.sensor_pos.manhattan_distance(other)
    }
}

fn parse_input(filename: &str) -> Vec<Sensor> {
    fn parse_value(input: &str) -> IResult<&str, isize> {
        let (input, sign) = opt(tag("-"))(input)?;
        let digit = map_res(digit1, |s: &str| s.parse::<isize>())(input);
        match sign {
            Some("-") => digit.map(|(input, value)| (input, -value)),
            _ => digit,
        }
    }

    fn parse_coord(input: &str) -> IResult<&str, Coord> {
        map(
            tuple((
                preceded(tag("x="), parse_value),
                preceded(tag(", y="), parse_value),
            )),
            |(x, y)| Coord::new(x, y),
        )(input)
    }

    fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
        map(
            tuple((
                preceded(tag("Sensor at "), parse_coord),
                preceded(tag(": closest beacon is at "), parse_coord),
            )),
            |(sensor, beacon)| Sensor::new(sensor, beacon),
        )(input)
    }

    fn parse_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
        separated_list1(newline, parse_sensor)(input)
    }

    let input = fs::read_to_string(filename).unwrap();
    let (_, pairs) = all_consuming(terminated(parse_sensors, multispace0))(input.as_str())
        .finish()
        .unwrap();
    pairs
}

pub fn count_positions_without_beacon_on_row(filename: &str, y: isize) -> usize {
    let sensors = parse_input(filename);
    let (min_x, max_x) = sensors
        .iter()
        .fold((0, 0), |(mut min_x, mut max_x), sensor| {
            min_x = min_x.min(min(sensor.sensor_pos.x, sensor.beacon_pos.x));
            max_x = max_x.max(max(sensor.sensor_pos.x, sensor.beacon_pos.x));
            (min_x, max_x)
        });
    (min_x..=max_x)
        .filter(|&x| {
            sensors.iter().any(|sensor| {
                sensor.within_distance(&Coord::new(x, y)) && sensor.beacon_pos != (x, y)
            })
        })
        .count()
}

pub fn tuning_frequency_of_distress_signal(
    filename: &str,
    min: isize,
    max: isize,
) -> Option<isize> {
    let sensors = parse_input(filename);
    for y in min..=max {
        let mut x = min;
        'x: loop {
            for sensor in sensors.iter() {
                let step = sensor.skip_distance(&Coord::new(x, y)) + 1;
                if step > 0 {
                    x += step;
                    if x > max {
                        break 'x;
                    }
                    continue 'x;
                }
            }
            return Some(x * 4000000 + y);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let count = count_positions_without_beacon_on_row("assets/example.txt", 10);
        assert_eq!(count, 26);
    }

    #[test]
    fn part2() {
        let frequency = tuning_frequency_of_distress_signal("assets/example.txt", 0, 20);
        assert_eq!(frequency, Some(56000011));
    }
}
