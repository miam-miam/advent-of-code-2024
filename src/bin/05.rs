use prse::parse;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<Vec<u32>>, HashMap<u32, Vec<u32>>) {
    let mut depends_on: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages = vec![];
    let mut first_stage = true;

    for line in input.lines() {
        if line.is_empty() {
            first_stage = false;
        } else if first_stage {
            let (a, b) = parse!(line, "{}|{}");
            depends_on.entry(b).or_default().push(a);
        } else {
            pages.push(parse!(line, "{:,:}"));
        }
    }

    (pages, depends_on)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pages, depends_on) = parse(input);

    let mut sum = 0;

    for page in pages {
        if check_page_valid(&page, &depends_on) {
            sum += page[page.len() / 2]
        }
    }
    Some(sum)
}

fn check_page_valid(page: &[u32], depends_on: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, elem) in page.iter().enumerate() {
        if let Some(dependencies) = depends_on.get(elem) {
            let currently_evaluated = &page[..i];
            if !dependencies
                .iter()
                .filter(|&d| page.contains(d))
                .all(|d| currently_evaluated.contains(d))
            {
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pages, depends_on) = parse(input);

    let mut sum = 0;

    for mut page in pages {
        if !check_page_valid(&page, &depends_on) {
            fix_page(&mut page, &depends_on);
            sum += page[page.len() / 2];
        }
    }

    Some(sum)
}

fn fix_page(page: &mut Vec<u32>, depends_on: &HashMap<u32, Vec<u32>>) {
    for (i, elem) in page.iter().enumerate() {
        if let Some(dependencies) = depends_on.get(elem) {
            let currently_evaluated = &page[..i];
            for d in dependencies.iter().filter(|&d| page.contains(d)) {
                if !currently_evaluated.contains(d) {
                    let pos = page.iter().position(|p| p == d).unwrap();
                    page.remove(pos);
                    page.insert(i, *d);
                    return fix_page(page, depends_on);
                }
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(143));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(123));
    }
}
