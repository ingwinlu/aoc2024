advent_of_code::solution!(7);

type Line = (u64, Vec<u64>);

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(':').unwrap();
            let numbers: Vec<u64> = end
                .split_whitespace()
                .map(|number| number.trim().parse().unwrap())
                .collect();
            (start.trim().parse().unwrap(), numbers)
        })
        .collect()
}

fn is_solvable_part1(
    expected_result: u64,
    mut numbers: impl Iterator<Item = u64> + Clone,
    intermediate_result: u64,
) -> bool {
    if intermediate_result > expected_result {
        return false;
    }
    if let Some(next_operand) = numbers.next() {
        is_solvable_part1(
            expected_result,
            numbers.clone(),
            intermediate_result + next_operand,
        ) || is_solvable_part1(expected_result, numbers, intermediate_result * next_operand)
    } else {
        expected_result == intermediate_result
    }
}

fn is_solvable_part2(
    expected_result: u64,
    mut numbers: impl Iterator<Item = u64> + Clone,
    intermediate_result: u64,
) -> bool {
    if intermediate_result > expected_result {
        return false;
    }
    if let Some(next_operand) = numbers.next() {
        is_solvable_part2(
            expected_result,
            numbers.clone(),
            intermediate_result + next_operand,
        ) || is_solvable_part2(
            expected_result,
            numbers.clone(),
            intermediate_result * next_operand,
        ) || is_solvable_part2(
            expected_result,
            numbers,
            intermediate_result * u64::pow(10, next_operand.to_string().len() as u32)
                + next_operand,
        )
    } else {
        expected_result == intermediate_result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse(input);
    let result = lines
        .into_iter()
        .filter_map(|(expected, numbers)| {
            let mut numbers_iter = numbers.into_iter();
            let first_number = numbers_iter.next().unwrap();
            is_solvable_part1(expected, numbers_iter, first_number).then_some(expected)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse(input);
    let result = lines
        .into_iter()
        .filter_map(|(expected, numbers)| {
            let mut numbers_iter = numbers.into_iter();
            let first_number = numbers_iter.next().unwrap();
            is_solvable_part2(expected, numbers_iter, first_number).then_some(expected)
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_last_line() {
        let (expected_result, numbers) = parse("292: 11 6 16 20")[0].to_owned();
        assert!(is_solvable_part1(expected_result, numbers.into_iter(), 0));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
