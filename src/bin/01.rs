use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut v1 = Vec::with_capacity(128);
    let mut v2 = Vec::with_capacity(128);
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        v1.push(parts.next().unwrap().parse::<u64>().unwrap());
        v2.push(parts.next().unwrap().parse::<u64>().unwrap());
    }
    v1.sort();
    v2.sort();
    let result = v1
        .into_iter()
        .zip(v2)
        .fold(0u64, |acc, (i1, i2)| acc + i1.abs_diff(i2));

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut v1 = Vec::with_capacity(128);
    let mut hm = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        v1.push(parts.next().unwrap().parse::<u64>().unwrap());

        let hashmap_key = parts.next().unwrap().parse::<u64>().unwrap();
        match hm.get_mut(&hashmap_key) {
            None => {
                hm.insert(hashmap_key, 1);
            }
            Some(old_value) => *old_value += 1,
        }
    }
    let result = v1
        .into_iter()
        .fold(0, |acc, i1| acc + i1 * hm.get(&i1).unwrap_or(&0));
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
