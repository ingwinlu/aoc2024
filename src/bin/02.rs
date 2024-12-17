advent_of_code::solution!(2);

fn check_safety(line: &str) -> bool {
    let mut values = Vec::new();
    for val in line.split_ascii_whitespace() {
        let current_value = val.parse::<u8>().unwrap();
        if values.is_empty() {
            values.push((current_value, 0));
            continue;
        }
        let last_inserted_diff = values.last().unwrap().0 as i32 - current_value as i32;
        if !(1..=3).contains(&last_inserted_diff.abs()) {
            return false;
        }
        values.push((current_value, last_inserted_diff));
        if values.len() > 2 {
            let mut iter = values.as_slice()[values.len() - 2..].iter();
            if !(iter.clone().all(|val| val.1 > 0) || iter.all(|val| val.1 < 0)) {
                return false;
            };
        }
    }
    true
}

fn check_safety_dampened(line: &str) -> bool {
    let values: Vec<u32> = line
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    is_safe(&values) || is_safe2(&values)
}

fn is_safe(v: &[u32]) -> bool {
    if !v.is_sorted() && !v.iter().rev().is_sorted() {
        return false;
    }
    for w in v.windows(2) {
        if (w[0] as i64 - w[1] as i64).abs() > 3 || (w[0] == w[1]) {
            return false;
        }
    }
    true
}

fn is_safe2(v: &[u32]) -> bool {
    if is_safe(&v[1..v.len()]) {
        return true;
    }
    for i in 1..v.len() {
        let (left, right) = v.split_at(i);
        let v2 = [left, &right[1..right.len()]].concat();
        if is_safe(&v2) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .fold(0, |acc, line| acc + check_safety(line) as u64),
    )
}

// taken / adapted from https://github.com/0xdea/aoc-2024-in-rust/blob/main/src/bin/02.rs
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .fold(0, |acc, line| acc + check_safety_dampened(line) as u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
