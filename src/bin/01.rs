use itertools::Itertools;
use prse::parse;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = input.lines().map(|l| -> (u32, u32) { parse!(l, "{}   {}")}).collect();
    first.sort();
    second.sort();
    Some(first.into_iter().zip_eq(second).map(|(f, s)| f.abs_diff(s)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = input.lines().map(|l| -> (u32, u32) { parse!(l, "{}   {}")}).collect();
    let first_counts = first.into_iter().counts();
    let second_counts = second.into_iter().counts();
    Some(first_counts.into_iter().map(|(num, count)| {
        let second_count = second_counts.get(&num).unwrap_or(&0);
        num as usize * count * second_count
    }).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(31));
    }
}
