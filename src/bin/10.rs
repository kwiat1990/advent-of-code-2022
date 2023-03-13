use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Signal {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            Some((_, x)) => Signal::Addx(x.parse::<i32>().unwrap()),
            _ => Signal::Noop,
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let signals = parse(input);
    let mut iter = signals.iter();
    let mut x = 1;
    let mut total = 0;
    let mut should_wait = false;
    let mut current_queue: VecDeque<i32> = VecDeque::new();

    for cycle in 1.. {
        if cycle % 40 == 20 {
            total += x * cycle;
        }

        if let Some(n) = current_queue.pop_front() {
            x += n;
        }

        if should_wait {
            should_wait = !should_wait;
            continue;
        }

        match iter.next() {
            Some(Signal::Noop) => {
                should_wait = false;
            }
            Some(Signal::Addx(x)) => {
                should_wait = true;
                current_queue.push_back(*x);
            }
            None => break,
        }
    }

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
