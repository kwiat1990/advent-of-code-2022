use std::collections::HashMap;

fn letter_score(x: char) -> usize {
    *('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, letter)| (letter, i + 1))
        .collect::<HashMap<char, usize>>()
        .get(&x)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(a, b)| {
                let result = a
                    .chars()
                    .find_map(|ch| b.contains(ch).then_some(ch))
                    .map(letter_score)
                    .unwrap();

                u32::try_from(result).unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| {
                let result = chunk[0]
                    .chars()
                    .find_map(|ch| (chunk[1].contains(ch) && chunk[2].contains(ch)).then_some(ch))
                    .map(letter_score)
                    .unwrap();

                u32::try_from(result).unwrap()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
