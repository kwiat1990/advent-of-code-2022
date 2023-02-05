fn group_input(input: &str) -> Vec<u32> {
    // First solution using imperative approach, which is a little faster:
    // let mut group: Vec<u32> = vec![0];
    // for value in input.lines() {
    //     match value.parse::<u32>() {
    //         Ok(num) => {
    //             if let Some(last_num) = group.last_mut() {
    //                 *last_num += num;
    //             }
    //         }
    //         Err(_) => group.push(0),
    //     }
    // }
    // group

    // Parsing produces something similar to [Some(123), None, Some(123), Some(123)],
    // which can be then easily splited to create groups and sum them up
    input
        .lines()
        .map(|val| val.parse::<u32>().ok())
        .collect::<Vec<Option<u32>>>()
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|num| num.unwrap_or_default()).sum::<u32>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    group_input(input).into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut groups = group_input(input);
    groups.sort_unstable();
    Some(groups.iter().rev().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(50101));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(115920));
    }
}
