use std::collections::HashMap;

advent_of_code::solution!(4);

type X = i64;
type Y = i64;
type Coordinate = (X, Y);

struct Grid {
    inner: HashMap<Coordinate, char>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid.insert((x as i64, y as i64), char);
            }
        }
        Self { inner: grid }
    }

    pub fn count_xmas(&self) -> u64 {
        self.find_starting_points('X')
            .map(|sp| self.find_xmas_from_starting_point(sp))
            .sum()
    }

    pub fn count_mas(&self) -> u64 {
        self.find_starting_points('A')
            .map(|sp| self.find_mas_from_starting_point(sp))
            .sum()
    }

    fn find_starting_points<'a>(
        &'a self,
        sp: char,
    ) -> Box<dyn Iterator<Item = &'a Coordinate> + 'a> {
        Box::new(
            self.inner
                .iter()
                .filter(move |(_, &v)| v == sp)
                .map(|(k, _)| k),
        )
    }

    fn find_xmas_from_starting_point(&self, coord: &Coordinate) -> u64 {
        let right = [
            (coord.0, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0 + 2, coord.1),
            (coord.0 + 3, coord.1),
        ];
        let left = [
            (coord.0, coord.1),
            (coord.0 - 1, coord.1),
            (coord.0 - 2, coord.1),
            (coord.0 - 3, coord.1),
        ];
        let up = [
            (coord.0, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 - 2),
            (coord.0, coord.1 - 3),
        ];
        let down = [
            (coord.0, coord.1),
            (coord.0, coord.1 + 1),
            (coord.0, coord.1 + 2),
            (coord.0, coord.1 + 3),
        ];
        let right_down = [
            (coord.0, coord.1),
            (coord.0 + 1, coord.1 + 1),
            (coord.0 + 2, coord.1 + 2),
            (coord.0 + 3, coord.1 + 3),
        ];
        let right_up = [
            (coord.0, coord.1),
            (coord.0 + 1, coord.1 - 1),
            (coord.0 + 2, coord.1 - 2),
            (coord.0 + 3, coord.1 - 3),
        ];
        let left_down = [
            (coord.0, coord.1),
            (coord.0 - 1, coord.1 + 1),
            (coord.0 - 2, coord.1 + 2),
            (coord.0 - 3, coord.1 + 3),
        ];
        let left_up = [
            (coord.0, coord.1),
            (coord.0 - 1, coord.1 - 1),
            (coord.0 - 2, coord.1 - 2),
            (coord.0 - 3, coord.1 - 3),
        ];

        let tester = |coords: &[Coordinate; 4]| {
            coords
                .map(|coord| self.inner.get(&coord))
                .iter()
                .zip(['X', 'M', 'A', 'S'])
                .all(|(&value, expected)| value == Some(&expected)) as u64
        };
        [
            right, left, up, down, right_down, right_up, left_up, left_down,
        ]
        .iter()
        .map(tester)
        .sum()
    }

    fn find_mas_from_starting_point(&self, coord: &Coordinate) -> u64 {
        let right_down = [
            (coord.0 - 1, coord.1 - 1),
            (coord.0, coord.1),
            (coord.0 + 1, coord.1 + 1),
        ];
        let right_up = [
            (coord.0 - 1, coord.1 + 1),
            (coord.0, coord.1),
            (coord.0 + 1, coord.1 - 1),
        ];
        let left_down = [
            (coord.0 + 1, coord.1 - 1),
            (coord.0, coord.1),
            (coord.0 - 1, coord.1 + 1),
        ];
        let left_up = [
            (coord.0 + 1, coord.1 + 1),
            (coord.0, coord.1),
            (coord.0 - 1, coord.1 - 1),
        ];

        let tester = |coords: &[Coordinate; 3]| {
            coords
                .map(|coord| self.inner.get(&coord))
                .iter()
                .zip(['M', 'A', 'S'])
                .all(|(&value, expected)| value == Some(&expected))
        };
        ([right_down, right_up, left_up, left_down]
            .iter()
            .map(tester)
            .filter(|&v| v)
            .count()
            == 2) as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    Some(grid.count_xmas())
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    Some(grid.count_mas())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
