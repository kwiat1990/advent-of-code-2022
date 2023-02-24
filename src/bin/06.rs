use std::collections::HashSet;

fn calc_position(input: &str, window_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window_size)
        .position(|x| x.iter().collect::<HashSet<_>>().len() == window_size)
        .map(|x| x + window_size)
}

pub fn part_one(input: &str) -> Option<usize> {
    const WINDOW_SIZE: usize = 4;
    calc_position(input, WINDOW_SIZE)
}

pub fn part_two(input: &str) -> Option<usize> {
    const WINDOW_SIZE: usize = 14;
    calc_position(input, WINDOW_SIZE)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
