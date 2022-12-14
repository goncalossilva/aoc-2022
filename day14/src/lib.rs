use grid::Grid;
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::Formatter;
use std::fs;
use std::iter::Iterator;
use std::ops::Index;

#[derive(Default, Clone, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Rock,
    Sand,
}

struct Cave {
    grid: Grid<Tile>,
    falling: Option<(usize, usize)>,
    entry: (usize, usize),
}

impl Cave {
    fn new(rows: usize, cols: usize, entry: (usize, usize)) -> Cave {
        Cave {
            grid: Grid::new(rows, cols),
            entry: (entry.0, entry.1),
            falling: None,
        }
    }

    fn is_within_bounds(&self, row: usize, col: usize) -> bool {
        row < self.grid.rows() && col < self.grid.cols()
    }

    fn add_floor(&mut self, distance: usize) {
        assert!(distance > 0);
        for _ in (0..(self.grid.rows() + distance) * 2 - self.entry.1).step_by(2) {
            self.grid.insert_col(0, vec![Tile::Empty; self.grid.rows()]);
            self.grid.push_col(vec![Tile::Empty; self.grid.rows()]);
            self.entry.1 += 1;
        }
        for _ in 1..distance {
            self.grid.push_row(vec![Tile::Empty; self.grid.cols()]);
        }
        self.grid.push_row(vec![Tile::Rock; self.grid.cols()]);
    }

    fn pour(&mut self) -> Result<(), ()> {
        if self.grid[self.entry.0][self.entry.1] == Tile::Empty {
            self.grid[self.entry.0][self.entry.1] = Tile::Sand;
            self.falling = Some(self.entry);
            Ok(())
        } else {
            Err(())
        }
    }

    fn step(&mut self) -> Result<Option<(usize, usize)>, ()> {
        if let Some((row, col)) = self.falling {
            let deltas: Vec<(usize, isize)> = vec![(1, 0), (1, -1), (1, 1)];
            for (drow, dcol) in deltas {
                let new_row = row + drow;
                let new_col = col.wrapping_add(dcol as usize);
                let bounded = self.is_within_bounds(new_row, new_col);
                if bounded && self.grid[new_row][new_col] != Tile::Empty {
                    continue;
                }
                self.grid[row][col] = Tile::Empty;
                return if bounded {
                    self.grid[new_row][new_col] = Tile::Sand;
                    self.falling = Some((new_row, new_col));
                    Ok(Some((row, col)))
                } else {
                    Err(())
                };
            }
        }
        Ok(None)
    }
}

impl Index<usize> for Cave {
    type Output = [Tile];

    #[inline]
    fn index(&self, idx: usize) -> &[Tile] {
        self.grid.index(idx)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::Sand => write!(f, "o"),
        }
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.rows() {
            for (col, tile) in self.grid.iter_row(row).enumerate() {
                if (row, col) == self.entry {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_cave(filename: &str) -> Cave {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines();

    let paths: Vec<Vec<(usize, usize)>> = lines
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split(',').collect_tuple().unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect()
        })
        .collect();

    let ((min_col, max_col), (_, max_row)) = paths.iter().flatten().fold(
        ((500, 500), (0, 0)),
        |((min_col, max_col), (_, max_row)), (col, row)| {
            (
                (min(min_col, *col), max(max_col, *col)),
                (0, max(max_row, *row)),
            )
        },
    );
    let width = max_col - min_col + 1;
    let height = max_row + 1;

    let mut cave = Cave::new(height, width, (0, 500 - min_col));
    for path in paths.iter() {
        path.iter()
            .tuple_windows()
            .for_each(|(&(col1, row1), &(col2, row2))| {
                for col in (col1..=col2).chain(col2..=col1) {
                    for row in (row1..=row2).chain(row2..=row1) {
                        cave.grid[row][col - min_col] = Tile::Rock;
                    }
                }
            });
    }

    cave
}

pub fn count_sand(filename: &str, surround_by_rocks: Option<usize>) -> usize {
    let mut cave = parse_cave(filename);
    if let Some(pad) = surround_by_rocks {
        cave.add_floor(pad);
    }
    'pour: loop {
        match cave.pour() {
            Ok(_) => loop {
                match cave.step() {
                    Ok(Some(_)) => (),
                    Ok(None) => {
                        break;
                    }
                    Err(_) => {
                        break 'pour;
                    }
                }
            },
            Err(_) => break 'pour,
        }
    }
    cave.grid.iter().filter(|tile| **tile == Tile::Sand).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let count = count_sand("assets/example.txt", None);
        assert_eq!(count, 24);
    }

    #[test]
    fn part2() {
        let count = count_sand("assets/example.txt", Some(2));
        assert_eq!(count, 93);
    }
}
