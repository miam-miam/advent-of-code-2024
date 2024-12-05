use array2d::Array2D;
use itertools::Itertools;

const DIFFS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (-1, 0),
    (-1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (1, -1),
];

fn get_iters(
    pos: (usize, usize),
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    DIFFS.into_iter().flat_map(move |(dx, dy)| {
        if !(0..max_x as i32).contains(&(dx * 3 + (pos.0 as i32)))
            || !(0..max_y as i32).contains(&(dy * 3 + (pos.1 as i32)))
        {
            return None;
        }
        Some((1..4).map(move |i| {
            (
                (i * dx + pos.0 as i32) as usize,
                (i * dy + pos.1 as i32) as usize,
            )
        }))
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let test = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let array = Array2D::from_rows(&test).unwrap();
    let max_x = array.column_len();
    let max_y = array.row_len();
    Some(
        array
            .enumerate_column_major()
            .filter(|(_, e)| **e == 'X')
            .map(|((x, y), _)| {
                get_iters((x, y), max_x, max_y)
                    .map(|iter| {
                        iter.map(|pos| array.get(pos.0, pos.1).unwrap())
                            .enumerate()
                            .all(|(i, c)| matches!((i, c), (0, 'M') | (1, 'A') | (2, 'S')))
                    })
                    .filter(|&b| b)
                    .count()
            })
            .sum(),
    )
}

fn get_x_iters(pos: (usize, usize), max_x: usize, max_y: usize) -> Option<[(usize, usize); 4]> {
    if [(1, 1), (-1, -1), (1, -1), (-1, 1)]
        .into_iter()
        .all(|(dx, dy)| {
            (0..max_x as i32).contains(&(dx + (pos.0 as i32)))
                && (0..max_y as i32).contains(&(dy + (pos.1 as i32)))
        })
    {
        Some(
            [(1, 1), (-1, -1), (1, -1), (-1, 1)]
                .map(|(dx, dy)| ((dx + pos.0 as i32) as usize, (dy + pos.1 as i32) as usize)),
        )
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let test = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let array = Array2D::from_rows(&test).unwrap();
    let max_x = array.column_len();
    let max_y = array.row_len();
    Some(
        array
            .enumerate_column_major()
            .filter(|(_, e)| **e == 'A')
            .filter_map(|((x, y), _)| get_x_iters((x, y), max_x, max_y))
            .filter(|dirs| {
                check_matches(&array, dirs[0], dirs[1]) && check_matches(&array, dirs[2], dirs[3])
            })
            .count(),
    )
}

fn check_matches(array: &Array2D<char>, a_pos: (usize, usize), b_pos: (usize, usize)) -> bool {
    let &a = array.get(a_pos.0, a_pos.1).unwrap();
    let &b = array.get(b_pos.0, b_pos.1).unwrap();
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
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
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(9));
    }
}
