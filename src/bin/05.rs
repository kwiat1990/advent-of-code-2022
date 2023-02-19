use std::collections::VecDeque;
struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl Step {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self {
            count,
            from: from - 1,
            to: to - 1,
        }
    }
}

fn proccess_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Step>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let rows: Vec<_> = stacks
        .lines()
        .filter(|line| line.contains(char::is_alphabetic))
        .flat_map(|line| {
            line.split('\n')
                .map(|x| x.chars().skip(1).step_by(4).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for _ in 0..rows[0].len() {
        stacks.push(VecDeque::new());
    }

    for row in rows.iter() {
        for (i, ch) in row.iter().enumerate() {
            if ch.is_whitespace() {
                continue;
            }
            if let Some(stack) = stacks.get_mut(i) {
                stack.push_back(*ch);
            }
        }
    }

    let steps: Vec<Step> = moves
        .lines()
        .map(|line| {
            let [count, from, to]: [usize; 3] = line
                .split(char::is_alphabetic)
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            Step::new(count, from, to)
        })
        .collect();

    (stacks, steps)
}

fn extract_output(stacks: Vec<VecDeque<char>>) -> Option<String> {
    let output = stacks
        .iter()
        .map(|x| x.front().unwrap())
        .fold(String::new(), |mut output, ch| {
            output.push(*ch);
            output
        });

    Some(output)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, steps) = proccess_input(input);

    for step in steps {
        for _ in 0..step.count {
            let moving_crate = stacks.get_mut(step.from).unwrap().pop_front().unwrap();
            stacks.get_mut(step.to).unwrap().push_front(moving_crate);
        }
    }

    extract_output(stacks)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, steps) = proccess_input(input);

    for step in steps {
        let mut items: VecDeque<char> = VecDeque::new();
        for _ in 0..step.count {
            let current_crate = stacks.get_mut(step.from).unwrap().pop_front().unwrap();
            items.push_back(current_crate);
        }
        let stack = stacks.get_mut(step.to).unwrap();
        let initial_items = stack.split_off(0);
        stack.extend(items);
        stack.extend(initial_items);
    }

    extract_output(stacks)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
