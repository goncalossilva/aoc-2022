use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn parse_map(filename: String) -> Grid<i8> {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines();

    let mut map: Grid<i8> = Grid::new(0, 0);
    for line in lines {
        map.push_row(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect_vec(),
        );
    }
    map
}

pub fn count_visible_trees_from_edges(filename: String) -> usize {
    let map = parse_map(filename);
    let rows = 0..map.rows();
    let cols = 0..map.cols();

    fn visible_trees_from_edge<T, U>(map: &Grid<i8>, rows: T, cols: U) -> Vec<(usize, usize)>
    where
        T: IntoIterator<Item = usize>,
        U: IntoIterator<Item = usize> + Clone,
    {
        let mut visible = Vec::new();
        for i in rows {
            let mut tallest: i8 = -1;
            for j in cols.clone() {
                let current = *map.get(i, j).unwrap();
                if current > tallest {
                    tallest = current;
                    visible.push((i, j));
                }
            }
        }
        visible
    }

    fn rev(vec: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        vec.iter().map(|(i, j)| (*j, *i)).collect_vec()
    }

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // Left to right.
    visible.extend(visible_trees_from_edge(&map, rows.clone(), cols.clone()));

    // Top to bottom.
    visible.extend(rev(visible_trees_from_edge(
        &map.transpose(),
        rows.clone(),
        cols.clone(),
    )));

    // Right to left.
    visible.extend(visible_trees_from_edge(
        &map,
        rows.clone(),
        cols.clone().rev(),
    ));

    // Bottom to top.
    visible.extend(rev(visible_trees_from_edge(
        &map.transpose(),
        rows,
        cols.rev(),
    )));

    visible.len()
}

pub fn highest_tree_scenic_score(filename: String) -> usize {
    let map = parse_map(filename);
    let mut scores = Grid::new(map.rows(), map.cols());

    fn scenic_score(map: &Grid<i8>, row: usize, col: usize) -> usize {
        let mut score = 1;
        let height = map[row][col];

        // Looking down.
        let mut i = row;
        while i < map.rows() - 1 {
            i += 1;
            if map[i][col] >= height {
                break;
            }
        }
        score *= i - row;

        // Looking up.
        let mut i = row;
        while i > 0 {
            i -= 1;
            if map[i][col] >= height {
                break;
            }
        }
        score *= row - i;

        // Looking right.
        let mut i = col;
        while i < map.cols() - 1 {
            i += 1;
            if map[row][i] >= height {
                break;
            }
        }
        score *= i - col;

        // Looking left.
        let mut i = col;
        while i > 0 {
            i -= 1;
            if map[row][i] >= height {
                break;
            }
        }
        score *= col - i;

        score
    }

    for i in 0..map.rows() {
        for j in 0..map.cols() {
            scores[i][j] = scenic_score(&map, i, j);
        }
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = count_visible_trees_from_edges("assets/example.txt".to_string());
        assert_eq!(result, 21);
    }

    #[test]
    fn part2() {
        let result = highest_tree_scenic_score("assets/example.txt".to_string());
        assert_eq!(result, 8);
    }
}
