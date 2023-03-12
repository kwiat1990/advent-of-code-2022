use std::ops::Range;

struct Grid {
    data: Vec<u32>,
    size: usize,
}

impl Grid {
    fn get_cell(&self, col: usize, row: usize) -> u32 {
        self.data[col + self.size * row]
    }

    fn get_row(&self, row: usize) -> Vec<u32> {
        self.data[(row * self.size)..((row + 1) * self.size)].to_vec()
    }

    fn get_col(&self, col: usize) -> Vec<u32> {
        (0..self.size)
            .map(|i| self.data[i * self.size + col])
            .collect()
    }

    fn get_from_directions(&self, col: usize, row: usize) -> [Vec<u32>; 4] {
        let full_row = self.get_row(row);
        let full_col = self.get_col(col);

        let (left, right) = full_row.split_at(col);
        let (top, bottom) = full_col.split_at(row);

        // seen from perspective of the current element, that's why reversed
        let left: Vec<u32> = left.iter().copied().rev().collect();
        let top: Vec<u32> = top.iter().copied().rev().collect();
        // without the current element itself
        let right = right[1..].to_vec();
        let bottom = bottom[1..].to_vec();

        [left, right, top, bottom]
    }
}

fn create_grid(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()))
        .collect()
}

fn cartesian_product(range: Range<usize>) -> Vec<(usize, usize)> {
    range
        .clone()
        .flat_map(|row| range.clone().map(move |col| (col, row)))
        .collect()
}

// After refactoring both solutions became >7x slower but the code is much more readable and concise.
pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid {
        size: input.lines().next().unwrap().len(),
        data: create_grid(input),
    };

    let edges_count = (grid.size - 1) * 4;

    let interior_count = cartesian_product(1..grid.size - 1)
        .into_iter()
        .map(|(col, row)| {
            let height = grid.get_cell(col, row);
            grid.get_from_directions(col, row)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count();

    Some(edges_count + interior_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid {
        size: input.lines().next().unwrap().len(),
        data: create_grid(input),
    };

    cartesian_product(1..grid.size - 1)
        .into_iter()
        .map(|(col, row)| {
            let height = grid.get_cell(col, row);
            grid.get_from_directions(col, row)
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
