struct Points {
    win: u32,
    loss: u32,
    draw: u32,
}

impl Points {
    fn new(base: u32) -> Self {
        Self {
            win: 6 + base,
            draw: 3 + base,
            loss: base,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rock = Points::new(1);
    let paper = Points::new(2);
    let scissors = Points::new(3);

    let mut score = 0;

    for line in input.lines() {
        let (oponent, player) = line.split_once(' ').unwrap();
        match (oponent, player) {
            ("A", "X") => score += rock.draw,
            ("B", "X") => score += rock.loss,
            ("C", "X") => score += rock.win,

            ("A", "Y") => score += paper.win,
            ("B", "Y") => score += paper.draw,
            ("C", "Y") => score += paper.loss,

            ("A", "Z") => score += scissors.loss,
            ("B", "Z") => score += scissors.win,
            ("C", "Z") => score += scissors.draw,

            (_, _) => score += 0,
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rock = Points::new(1);
    let paper = Points::new(2);
    let scissors = Points::new(3);

    let mut score = 0;

    for line in input.lines() {
        let (oponent, player) = line.split_once(' ').unwrap();
        match (oponent, player) {
            ("A", "X") => score += scissors.loss,
            ("B", "X") => score += rock.loss,
            ("C", "X") => score += paper.loss,

            ("A", "Y") => score += rock.draw,
            ("B", "Y") => score += paper.draw,
            ("C", "Y") => score += scissors.draw,

            ("A", "Z") => score += paper.win,
            ("B", "Z") => score += scissors.win,
            ("C", "Z") => score += rock.win,

            (_, _) => score += 0,
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(38));
    }
}
