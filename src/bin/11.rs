#[derive(Debug, Clone)]
struct Monkey<'a> {
    id: u64,
    items: Vec<u64>,
    inspections_count: u64,
    operation: Vec<&'a str>,
    test: TestCase,
}

#[derive(Debug, Clone)]
struct TestCase {
    divisor: u64,
    true_target: u64,
    false_target: u64,
}

impl<'a> Monkey<'a> {
    fn new(id: u64, items: Vec<u64>, operation: Vec<&'a str>, test: TestCase) -> Self {
        Self {
            id,
            items,
            inspections_count: 0,
            operation,
            test,
        }
    }

    fn make_inspection(&mut self) -> u64 {
        self.items.reverse();
        let lh = self.items.pop().unwrap();

        let rh = if self.operation[2] == "old" {
            lh
        } else {
            self.operation[2].parse::<u64>().unwrap()
        };

        self.increase_inspection_count();

        match self.operation[1] {
            "*" => lh * rh,
            "+" => lh + rh,
            _ => panic!("not recognized operator sign"),
        }
    }

    fn increase_inspection_count(&mut self) {
        self.inspections_count += 1;
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            let id = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let items = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(',')
                .map(|ch| ch.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operation = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: new = ")
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();

            let divisor = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let true_target = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let false_target = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            Monkey::new(
                id,
                items,
                operation,
                TestCase {
                    divisor,
                    true_target,
                    false_target,
                },
            )
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            for _ in 0..monkeys[monkey].items.len() {
                let worry_level = monkeys[monkey].make_inspection() / 3;

                let TestCase {
                    divisor,
                    true_target,
                    false_target,
                } = monkeys[monkey].test;

                let target_monkey = monkeys
                    .iter_mut()
                    .find(|target| {
                        let source = if worry_level % divisor == 0 {
                            true_target
                        } else {
                            false_target
                        };
                        target.id == source
                    })
                    .unwrap();
                target_monkey.add_item(worry_level);
            }
        }
    }

    let mut results = monkeys
        .iter()
        .map(|monkey| monkey.inspections_count)
        .collect::<Vec<u64>>();

    results.sort();

    let monkey_businnes = results.last().unwrap() * results.get(results.len() - 2).unwrap();

    Some(monkey_businnes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test.divisor).product();
    for _ in 0..10_000 {
        for monkey in 0..monkeys.len() {
            for _ in 0..monkeys[monkey].items.len() {
                let worry_level = monkeys[monkey].make_inspection() % common_multiple;

                let TestCase {
                    divisor,
                    true_target,
                    false_target,
                } = monkeys[monkey].test;

                let target_monkey = monkeys
                    .iter_mut()
                    .find(|target| {
                        let source = if worry_level % divisor == 0 {
                            true_target
                        } else {
                            false_target
                        };
                        target.id == source
                    })
                    .unwrap();
                target_monkey.add_item(worry_level);
            }
        }
    }

    let mut results = monkeys
        .iter()
        .map(|monkey| monkey.inspections_count)
        .collect::<Vec<u64>>();

    results.sort_unstable();

    let monkey_businnes = results.iter().rev().take(2).product();

    Some(monkey_businnes)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
