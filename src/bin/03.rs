use itertools::Itertools;
use prse::parse;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| {
                let (a, b): (u32, u32) = parse!(c.get(0).unwrap().as_str(), "mul({},{})");
                a * b
            })
            .sum(),
    )
}

// Utter mess 🤦 why write 1 regex when you can write 2.
pub fn part_two(input: &str) -> Option<u32> {
    let re1 = Regex::new(r"do(n't)?\(\)").unwrap();
    let mut start = Some(0);
    let mut ranges = re1
        .captures_iter(input)
        .filter_map(|c| {
            let m = c.get(0).unwrap();
            if c.get(1).is_none() {
                if start.is_none() {
                    start = Some(m.end());
                }
                None
            } else if let Some(s) = start {
                let a = (s, m.start());
                start = None;
                Some(a)
            } else {
                None
            }
        })
        .collect_vec();

    if let Some(s) = start {
        ranges.push((s, input.len()))
    }

    let re = regex::bytes::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        ranges
            .into_iter()
            .map(|(s, e)| {
                re.captures_iter(&input.as_bytes()[s..e])
                    .map(|c| {
                        c.iter()
                            .skip(1)
                            .flat_map(|m| {
                                m.map(|m| -> u32 {
                                    parse!(std::str::from_utf8(m.as_bytes()).unwrap(), "{}")
                                })
                            })
                            .product::<u32>()
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(201));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(88));
    }
}
