use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<usize> {
    let start = Position { x: 0, y: 0 };
    let mut seen_pos = HashSet::new();

    let mut head = start;
    let mut tail = start;

    seen_pos.insert(start);

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("invalid direction"),
            }

            let current_gap = Position {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };

            let should_catch_up = current_gap.x.abs() > 1 || current_gap.y.abs() > 1;

            if should_catch_up {
                // signum converts positive numbers to 1, negative to -1
                tail.x += current_gap.x.signum();
                tail.y += current_gap.y.signum();
                seen_pos.insert(tail);
            }
        }
    }

    Some(seen_pos.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let start = Position { x: 0, y: 0 };
    let mut seen_pos = HashSet::new();

    let mut knots = [start; 10];
    seen_pos.insert(start);

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => knots[0].y -= 1,
                "D" => knots[0].y += 1,
                "L" => knots[0].x -= 1,
                "R" => knots[0].x += 1,
                _ => panic!("invalid direction"),
            }

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let mut tail = &mut knots[i];

                let current_gap = Position {
                    x: head.x - tail.x,
                    y: head.y - tail.y,
                };

                let should_catch_up = current_gap.x.abs() > 1 || current_gap.y.abs() > 1;

                if should_catch_up {
                    // signum converts positive numbers to 1, negative to -1
                    tail.x += current_gap.x.signum();
                    tail.y += current_gap.y.signum();
                    if i == 9 {
                        seen_pos.insert(*tail);
                    }
                }
            }
        }
    }

    Some(seen_pos.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
