use std::ops::Range;

fn create_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_from_directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = &grid[y];
    let column: Vec<u32> = grid.iter().map(|row| row[x]).collect();

    let (left, right) = row.split_at(x);
    let (top, bottom) = column.split_at(y);

    // seen from perspective of the current element, that's why reversed
    let left: Vec<u32> = left.iter().copied().rev().collect();
    let top: Vec<u32> = top.iter().copied().rev().collect();
    // without the current element itself
    let right = right[1..].to_vec();
    let bottom = bottom[1..].to_vec();

    [left, right, top, bottom]
}

fn cartesian_product(range: Range<usize>) -> Vec<(usize, usize)> {
    range
        .clone()
        .flat_map(|y| range.clone().map(move |x| (x, y)))
        .collect()
}

// After refactoring both solutions became >7x slower but the code is much more readable and concise.
pub fn part_one(input: &str) -> Option<usize> {
    let grid = create_grid(input);
    let edges_count = (grid.len() - 1) * 4;

    let interior_count = cartesian_product(1..grid.len() - 1)
        .iter()
        .map(|&(x, y)| {
            let height = grid[x][y];
            get_from_directions(&grid, y, x)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count();

    Some(edges_count + interior_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = create_grid(input);

    cartesian_product(1..grid.len() - 1)
        .iter()
        .map(|&(x, y)| {
            let height = grid[x][y];
            get_from_directions(&grid, y, x)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|count| count + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product()
        })
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
