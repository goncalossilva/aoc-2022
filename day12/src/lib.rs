use grid::Grid;
use pathfinding::prelude::astar;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Map {
    heightmap: Grid<i8>,
    start: Pos,
    end: Pos,
}

impl Pos {
    fn distance(&self, pos: Pos) -> usize {
        self.0.abs_diff(pos.0) + self.1.abs_diff(pos.1)
    }
}

impl Map {
    fn new(heightmap: Grid<i8>, start: Pos, end: Pos) -> Map {
        Map {
            heightmap,
            start,
            end,
        }
    }

    fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let height = self.heightmap[pos.0][pos.1];
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas
            .iter()
            .map(|&(dx, dy)| {
                let row = pos.0.wrapping_add(dx as usize);
                let col = pos.1.wrapping_add(dy as usize);
                (row, col)
            })
            .filter(|&(row, col)| row < self.heightmap.rows() && col < self.heightmap.cols())
            .filter(|&(row, col)| self.heightmap[row][col] - height <= 1)
            .map(|(x, y)| Pos(x, y))
            .collect()
    }
}

fn parse_map(filename: String) -> Map {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines();

    let mut heightmap: Grid<i8> = Grid::new(0, 0);
    let mut start: Pos = Pos(0, 0);
    let mut end: Pos = Pos(0, 0);
    for (row, line) in lines.enumerate() {
        heightmap.push_row(
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let height = match c {
                        'S' => {
                            start = Pos(row, col);
                            'a'
                        }
                        'E' => {
                            end = Pos(row, col);
                            'z'
                        }
                        c => c,
                    };
                    height as i8
                })
                .collect(),
        );
    }
    Map::new(heightmap, start, end)
}

fn minimum_steps_to_destination_from_pos(map: &Map, pos: Pos) -> Option<usize> {
    Some(
        astar(
            &pos,
            |pos| {
                map.neighbors(pos)
                    .iter()
                    .map(|&p| (p, 1))
                    .collect::<Vec<(Pos, usize)>>()
            },
            |pos| pos.distance(map.end) / 3,
            |&pos| pos == map.end,
        )?
        .1,
    )
}

pub fn minimum_steps_to_destination_from_start(filename: &str) -> usize {
    let map = parse_map(filename.to_string());
    let pos = map.start;
    minimum_steps_to_destination_from_pos(&map, pos).unwrap()
}

pub fn minimum_steps_to_destination_from_best_position(filename: &str) -> usize {
    let map = parse_map(filename.to_string());
    map.heightmap
        .iter()
        .enumerate()
        .filter(|(_, &height)| height == 'a' as i8)
        .map(|(i, _)| Pos(i / map.heightmap.cols(), i % map.heightmap.cols()))
        .map(|pos| minimum_steps_to_destination_from_pos(&map, pos).unwrap_or(usize::MAX))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let steps = minimum_steps_to_destination_from_start("assets/example.txt");
        assert_eq!(steps, 31);
    }

    #[test]
    fn part2() {
        let steps = minimum_steps_to_destination_from_best_position("assets/example.txt");
        assert_eq!(steps, 29);
    }
}
