#[derive(Debug, PartialEq)]
enum Signal {
    Noop,
    Addx(i16),
}

fn parse(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            Some((_, x)) => Signal::Addx(x.parse::<i16>().unwrap()),
            _ => Signal::Noop,
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i16> {
    let signals = parse(input);
    let mut x = 1;
    let mut total = 0;
    let mut cycle = 1;

    signals.iter().for_each(|signal| {
        if cycle % 40 == 20 {
            total += x * cycle;
        }

        cycle += 1;

        if let Signal::Addx(num) = signal {
            if cycle % 40 == 20 {
                total += x * cycle;
            }
            x += num;
            cycle += 1;
        }
    });

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
