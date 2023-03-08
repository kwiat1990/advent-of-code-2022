fn create_grid(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut columns: Vec<Vec<u32>> = vec![];
    let mut rows: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let chars: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        rows.push(chars.clone());
        for (i, item) in chars.iter().enumerate() {
            match columns.get_mut(i) {
                Some(x) => x.push(*item),
                None => columns.push(vec![*item]),
            }
        }
    }

    (columns, rows)
}

fn count_edges(grid: (&Vec<Vec<u32>>, &Vec<Vec<u32>>)) -> usize {
    let columns = (grid.0.len() - 2) * 2; // remove item from top and bottom corners (-2)
    let rows = grid.1.len() * 2;
    columns + rows
}

pub fn part_one(input: &str) -> Option<usize> {
    let (columns, rows) = create_grid(input);
    let mut visible_items = count_edges((&columns, &rows));

    let interior_rows = &rows[1..rows.len() - 1];

    for (i, row) in interior_rows.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if y == 0 || y == row.len() - 1 {
                continue;
            }

            let has_bigger = |slice: &[u32]| -> bool { slice.iter().any(|x| x >= item) };

            let invisible_in_row = has_bigger(&row[..y]) && has_bigger(&row[y + 1..]);
            let invisible_in_column =
                has_bigger(&columns[y][..=i]) && has_bigger(&columns[y][i + 2..]);

            if !invisible_in_row || !invisible_in_column {
                visible_items += 1;
            }
        }
    }

    Some(visible_items)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
