use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = b'^',
    Right = b'>',
    Down = b'v',
    Left = b'<',
}

impl Direction {
    const fn new() -> Self {
        Self::Up
    }

    const fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn move_guard(x: &mut i32, y: &mut i32, dir: Direction) {
    match dir {
        Direction::Up => *y -= 1,
        Direction::Right => *x += 1,
        Direction::Down => *y += 1,
        Direction::Left => *x -= 1,
    };
}

fn color_path(mut input: Vec<Vec<char>>) -> Option<usize> {
    let mut count = 1;

    let xlen = input[0].len();
    let ylen = input.len();

    let mut pos = Position::new(0, 0);
    let mut dir = Direction::new();
    let mut visited = HashSet::new();

    #[allow(clippy::needless_range_loop)]
    for x in 0..xlen {
        for y in 0..ylen {
            if input[y][x] == '^' {
                input[y][x] = 'X';
                (pos.x, pos.y) = (x as i32, y as i32);
            }
        }
    }

    loop {
        // Return None if guard gets stuck in a loop
        if visited.contains(&(pos.x, pos.y, dir)) {
            return None;
        }

        // Otherwise keep moving
        visited.insert((pos.x, pos.y, dir));
        move_guard(&mut pos.x, &mut pos.y, dir);

        // Exit loop if guard leaves the area
        if pos.x < 0 || pos.y < 0 || pos.x >= xlen as i32 || pos.y >= ylen as i32 {
            break;
        }

        #[allow(clippy::match_on_vec_items)]
        match input[pos.y as usize][pos.x as usize] {
            '#' => {
                dir = dir.invert();
                move_guard(&mut pos.x, &mut pos.y, dir);
                dir = dir.turn_right();
            }
            c if c != 'X' => {
                input[pos.y as usize][pos.x as usize] = 'X';
                count += 1;
            }
            _ => (),
        }
    }
    Some(count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    color_path(input).map(|val| val as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut answer = 0;

    let xlen = input[0].len();
    let ylen = input.len();

    // Brute force approach;)
    for x in 0..xlen {
        for y in 0..ylen {
            let mut test_input = input.clone();
            // The new obstruction can't be placed at the guard's starting position
            if input[y][x] != '^' {
                test_input[y][x] = '#';
                if color_path(test_input).is_none() {
                    answer += 1;
                }
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
