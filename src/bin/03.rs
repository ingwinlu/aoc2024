use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{fold_many0, many0, many_till},
    sequence::delimited,
    IResult, Parser,
};

advent_of_code::solution!(3);

fn from_numeric_str(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse::<u16>()
}

// parse 2,4
fn parse_pair(input: &str) -> IResult<&str, (u16, u16)> {
    let (input, d1) = map_res(digit1, from_numeric_str).parse(input)?;
    let (input, _) = char(',')(input)?;
    let (input, d2) = map_res(digit1, from_numeric_str).parse(input)?;
    Ok((input, (d1, d2)))
}

#[derive(Debug)]
enum Command {
    Mul(u64),
    Do,
    Dont,
    Nop,
}

/// parse things like mul(2,4)
fn parse_mul(input: &str) -> IResult<&str, u64> {
    let (input, pair) = delimited(tag("mul("), parse_pair, tag(")")).parse(input)?;
    Ok((input, pair.0 as u64 * pair.1 as u64))
}

fn parse_mul2(input: &str) -> IResult<&str, Command> {
    let (input, pair) = delimited(tag("mul("), parse_pair, tag(")")).parse(input)?;
    Ok((input, Command::Mul(pair.0 as u64 * pair.1 as u64)))
}

/// parse things like xmul(2,4) dropping the x
fn parse_until_mul(input: &str) -> IResult<&str, u64> {
    let (input, result) = many_till(take(1u8), parse_mul).parse(input)?;
    Ok((input, result.1))
}

fn parse_agg_mul(input: &str) -> IResult<&str, u64> {
    fold_many0(parse_until_mul, || 0u64, |acc, item| acc + item).parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Command::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Command::Dont))
}

fn parse_nop(input: &str) -> IResult<&str, Command> {
    let (input, _) = take(1u8)(input)?;
    Ok((input, Command::Nop))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_mul2, parse_do, parse_dont, parse_nop)).parse(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_command).parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_agg_mul(input).unwrap();
    Some(result.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, commands) = parse_commands(input).unwrap();
    let mut agg = 0;
    let mut enabled = true;
    for command in commands {
        match command {
            Command::Nop => continue,
            Command::Do => enabled = true,
            Command::Dont => enabled = false,
            Command::Mul(m) if enabled => agg += m,
            Command::Mul(_) => continue,
        }
    }
    Some(agg)
}

#[cfg(test)]
mod tests {
    use nom::error::Error;

    use super::*;

    #[test]
    fn test_pair_ok() {
        let result = parse_pair("123,321");
        assert_eq!(result, Ok(("", (123, 321))));
    }

    #[test]
    fn test_pair_invalid() {
        let error = parse_pair("12a3,321").unwrap_err();
        assert_eq!(
            error,
            nom::Err::Error(Error::new("a3,321", nom::error::ErrorKind::Char))
        );
    }

    #[test]
    fn test_mul() {
        let result = parse_mul("mul(2,4)");
        assert_eq!(result, Ok(("", 8)));
    }

    #[test]
    fn test_mul_extra() {
        let result = parse_mul("asdfmul(2,4)");
        assert_eq!(
            result,
            Err(nom::Err::Error(Error::new(
                "asdfmul(2,4)",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
