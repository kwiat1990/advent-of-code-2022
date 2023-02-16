pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            let a1 = a1.parse::<u32>().unwrap();
            let a2 = a2.parse::<u32>().unwrap();
            let b1 = b1.parse::<u32>().unwrap();
            let b2 = b2.parse::<u32>().unwrap();

            a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2

            // A bit slower but cleaner approach would be:
            // let (left, right) = ((a1..=a2), (b1..=b2));
            // left.clone().all(|n| right.contains(&n)) || right.clone().all(|n| left.contains(&n))
        })
        .count();

    Some(pairs as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            let a1 = a1.parse::<u32>().unwrap();
            let a2 = a2.parse::<u32>().unwrap();
            let b1 = b1.parse::<u32>().unwrap();
            let b2 = b2.parse::<u32>().unwrap();

            a1 <= b2 && a2 >= b1
            // Or:
            // a1.max(b1) <= b2.min(a2)

            // A bit slower but cleaner approach would be:
            // let (left, right) = ((a1..=a2), (b1..=b2));
            // left.clone().any(|n| right.contains(&n)) || right.clone().any(|n| left.contains(&n))
        })
        .count();

    Some(pairs as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
