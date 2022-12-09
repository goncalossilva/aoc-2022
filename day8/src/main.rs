use day8::{count_visible_trees_from_edges, highest_tree_scenic_score};

fn main() {
    println!(
        "Part one: {}",
        count_visible_trees_from_edges("day8/assets/input.txt".to_string())
    );

    println!(
        "Part two: {}",
        highest_tree_scenic_score("day8/assets/input.txt".to_string())
    );
}
