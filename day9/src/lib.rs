use std::collections::HashSet;
use std::fs;
use vec1::Vec1;

macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct Segment {
    head_pos: Position,
    tail_pos: Position,
    head_visited: HashSet<Position>,
    tail_visited: HashSet<Position>,
}

struct Rope {
    segments: Vec1<Segment>,
}

impl Position {
    fn positions_around(&self) -> [Position; 8] {
        [
            pos!(self.x + 1, self.y),
            pos!(self.x + 1, self.y + 1),
            pos!(self.x, self.y + 1),
            pos!(self.x - 1, self.y + 1),
            pos!(self.x - 1, self.y),
            pos!(self.x - 1, self.y - 1),
            pos!(self.x, self.y - 1),
            pos!(self.x + 1, self.y - 1),
        ]
    }

    fn manhattan_distance(&self, other: &Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Segment {
    fn new() -> Segment {
        Segment {
            head_pos: pos!(0, 0),
            tail_pos: pos!(0, 0),
            head_visited: HashSet::from([pos!(0, 0)]),
            tail_visited: HashSet::from([pos!(0, 0)]),
        }
    }

    fn move_to(&mut self, x: isize, y: isize) {
        self.head_pos = pos!(x, y);
        self.head_visited.insert(self.head_pos);

        // Move tail.
        if self.tail_pos == self.head_pos {
            return;
        }
        let positions_around_head = self.head_pos.positions_around();
        if positions_around_head.contains(&self.tail_pos) {
            return;
        }

        let pos = positions_around_head
            .iter()
            .filter(|pos| {
                if self.tail_pos.x == self.head_pos.x || self.tail_pos.y == self.head_pos.y {
                    // Move straight.
                    pos.x == self.tail_pos.x || pos.y == self.tail_pos.y
                } else {
                    // Move diagonally.
                    pos.x != self.tail_pos.x && pos.y != self.tail_pos.y
                }
            })
            .min_by_key(|pos| self.tail_pos.manhattan_distance(pos))
            .unwrap();
        self.tail_pos = *pos;
        self.tail_visited.insert(self.tail_pos);
    }
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            segments: Vec1::try_from_vec(vec![Segment::new(); size]).unwrap(),
        }
    }

    fn head_pos(&self) -> Position {
        self.segments.first().head_pos
    }

    fn tail_pos(&self) -> Position {
        self.segments.last().tail_pos
    }

    fn head_visited(&self) -> &HashSet<Position> {
        &self.segments.first().head_visited
    }

    fn tail_visited(&self) -> &HashSet<Position> {
        &self.segments.last().tail_visited
    }

    fn move_facing(&mut self, direction: Direction) {
        let mut segments_iter = self.segments.iter_mut();
        let mut current = segments_iter.next().unwrap();
        let position = match direction {
            Direction::Up => pos!(current.head_pos.x, current.head_pos.y - 1),
            Direction::Right => pos!(current.head_pos.x + 1, current.head_pos.y),
            Direction::Down => pos!(current.head_pos.x, current.head_pos.y + 1),
            Direction::Left => pos!(current.head_pos.x - 1, current.head_pos.y),
        };
        current.move_to(position.x, position.y);

        for segment in segments_iter {
            segment.move_to(current.tail_pos.x, current.tail_pos.y);
            current = segment
        }
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_x = |pos: &&Position| pos.x;
        let key_y = |pos: &&Position| pos.y;
        let min_x = self.head_visited().iter().min_by_key(key_x).unwrap().x;
        let max_x = self.head_visited().iter().max_by_key(key_x).unwrap().x;
        let min_y = self.head_visited().iter().min_by_key(key_y).unwrap().y;
        let max_y = self.head_visited().iter().max_by_key(key_y).unwrap().y;
        for i in min_y..=max_y {
            for j in min_x..=max_x {
                let pos = pos!(j, i);
                if self.head_pos() == pos {
                    write!(f, "H")?;
                } else {
                    let knot = self
                        .segments
                        .iter()
                        .enumerate()
                        .find(|(_, segment)| segment.head_pos == pos);
                    match knot {
                        Some((index, _)) => write!(f, "{}", index),
                        None => {
                            if self.tail_pos() == pos {
                                write!(f, "T")
                            } else if i == 0 && j == 0 {
                                write!(f, "s")
                            } else if self.tail_visited().contains(&pos) {
                                write!(f, "#")
                            } else {
                                write!(f, ".")
                            }
                        }
                    }?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_rope(filename: String, segment_count: usize) -> Rope {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines();

    let mut rope = Rope::new(segment_count);
    for line in lines {
        let mut parts = line.split_whitespace();
        let direction = match parts.next() {
            Some("U") => Direction::Up,
            Some("R") => Direction::Right,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some(direction) => panic!("Unsupported direction: {}", direction),
            None => panic!("Missing direction"),
        };
        let steps = parts.next().unwrap().parse::<isize>().unwrap();

        for _ in 0..steps {
            rope.move_facing(direction);
        }
    }
    return rope;
}

pub fn count_tail_visited_positions(filename: String, segment_count: usize) -> usize {
    let rope = parse_rope(filename, segment_count);
    rope.tail_visited().len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "assets/example_small.txt",
        1,
        "
..##..
...##.
.TH##.
....#.
s###..
    "
    )]
    #[test_case(
        "assets/example_small.txt",
        9,
        "
......
......
.1H3..
.5....
6.....
    "
    )]
    #[test_case(
        "assets/example_large.txt",
        9,
        "
H.........................
1.........................
2.........................
3.........................
4.........................
5.........................
6.........................
7.........................
8.........................
T.........................
#.............###.........
#............#...#........
.#..........#.....#.......
..#..........#.....#......
...#........#.......#.....
....#......s.........#....
.....#..............#.....
......#............#......
.......#..........#.......
........#........#........
.........########.........
    "
    )]
    fn rope_fmt(filename: &str, segment_count: usize, expected: &str) {
        let rope = parse_rope(filename.to_string(), segment_count);
        assert_eq!(rope.to_string().trim(), expected.trim());
    }

    #[test_case(1, 13)]
    #[test_case(9, 1)]
    fn part1(segment_count: usize, expected: usize) {
        let count =
            count_tail_visited_positions("assets/example_small.txt".to_string(), segment_count);
        assert_eq!(count, expected)
    }

    #[test]
    fn part2() {
        let count = count_tail_visited_positions("assets/example_large.txt".to_string(), 9);
        assert_eq!(count, 36)
    }
}
