use std::collections::HashMap;

advent_of_code::solution!(5);

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> (Rules, Vec<Vec<&str>>) {
    let mut parse_rules = true;
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let (pre, post) = line.split_once('|').unwrap();
            match rules.get_mut(pre) {
                None => {
                    rules.insert(pre, vec![post]);
                }
                Some(existing) => existing.push(post),
            }
        } else {
            let pages: Vec<_> = line.split(",").collect();
            updates.push(pages);
        }
    }
    (rules, updates)
}

fn check_rules_for_update(rules: &Rules, update: &Vec<&str>) -> bool {
    for (i, &page) in update.iter().enumerate().skip(1) {
        let preceding_pages = &update[0..i];
        let pages_that_follow = match rules.get(page) {
            None => {
                continue;
            }
            Some(rules) => rules,
        };
        for &follwing_page in pages_that_follow.iter() {
            for &preceding_pages in preceding_pages {
                if preceding_pages == follwing_page {
                    return false;
                }
            }
        }
    }
    true
}

fn sort_incorrect(rules: &Rules, update: &mut Vec<&str>) {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..update.len() - 1 {
            if let Some(rule) = rules.get(update[i + 1]) {
                if rule.contains(&update[i]) {
                    update.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse(input);
    Some(
        updates
            .iter()
            .map(|update| match check_rules_for_update(&rules, update) {
                true => update[update.len() / 2].parse().unwrap(),
                false => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, mut updates) = parse(input);
    Some(
        updates
            .iter_mut()
            .map(|update| match check_rules_for_update(&rules, update) {
                true => 0,
                false => {
                    sort_incorrect(&rules, update);
                    update[update.len() / 2].parse().unwrap()
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
