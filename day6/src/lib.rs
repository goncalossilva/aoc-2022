use std::fs;

pub fn index_of_marker(filename: String, window_size: usize) -> Option<usize> {
    let input = fs::read_to_string(filename).unwrap();

    // Use the chars method to iterate over the characters in the string
    for (i, _) in input.chars().enumerate() {
        if i + window_size >= input.len() {
            break;
        }

        let window = &input[i..i + window_size];

        if window
            .chars()
            .enumerate()
            .any(|(i, c)| window[i + 1..].contains(c))
        {
            continue;
        }

        return Some(window_size + i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("assets/example1.txt", 7)]
    #[test_case("assets/example2.txt", 5)]
    #[test_case("assets/example3.txt", 6)]
    #[test_case("assets/example4.txt", 10)]
    #[test_case("assets/example5.txt", 11)]
    fn part1(filename: &str, index: usize) {
        let result = index_of_marker(filename.to_string(), 4);
        assert_eq!(result, Some(index));
    }

    #[test_case("assets/example1.txt", 19)]
    #[test_case("assets/example2.txt", 23)]
    #[test_case("assets/example3.txt", 23)]
    #[test_case("assets/example4.txt", 29)]
    #[test_case("assets/example5.txt", 26)]
    fn part2(filename: &str, index: usize) {
        let result = index_of_marker(filename.to_string(), 14);
        assert_eq!(result, Some(index));
    }
}
