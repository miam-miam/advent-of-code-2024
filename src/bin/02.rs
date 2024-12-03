use itertools::Itertools;
use prse::parse;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|reports| {
                let levels = parse!(reports, "{: :0}").filter_map(|r| r.ok());
                let mut increasing = None;
                levels
                    .tuple_windows()
                    .all(|(a, b): (i32, i32)| match increasing {
                        None => {
                            increasing = Some(a < b);
                            (1..=3).contains(&a.abs_diff(b))
                        }
                        Some(true) => (1..=3).contains(&(b - a)),
                        Some(false) => (1..=3).contains(&(a - b)),
                    })
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|reports| {
                let levels = parse!(reports, "{: :0}").filter_map(|r| r.ok());
                let mut increasing = None;
                let mut tolerate = false;
                let mut last_value = None;
                levels
                    .tuple_windows()
                    .enumerate()
                    .all(|(i, (a, b)): (usize, (i32, i32))| {
                        let value = check_val(&mut increasing, a, b);
                        if !value && !tolerate {
                            tolerate = true;
                            if i == 0 || i == 1 {
                                increasing = None
                            }
                            match last_value {
                                None => true,
                                Some(v) => check_val(&mut increasing, v, b),
                            }
                        } else {
                            last_value = Some(a);
                            value
                        }
                    })
            })
            .count(),
    )
}

fn check_val(increasing: &mut Option<bool>, a: i32, b: i32) -> bool {
    match increasing {
        None => {
            *increasing = Some(a < b);
            (1..=3).contains(&a.abs_diff(b))
        }
        Some(true) => (1..=3).contains(&(b - a)),
        Some(false) => (1..=3).contains(&(a - b)),
    }
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
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(4));
    }
}
