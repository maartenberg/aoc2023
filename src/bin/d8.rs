use nom::combinator::all_consuming;
use nom::{
    character::complete::alpha1,
    sequence::{separated_pair, tuple},
};
use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::value,
    multi::many1,
    sequence::{delimited, terminated},
    IResult,
};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

#[derive(Clone, Debug)]
enum Direction {
    Right,
    Left,
}

type Connections<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    terminated(
        many1(alt((
            value(Direction::Right, char('R')),
            value(Direction::Left, char('L')),
        ))),
        newline,
    )(input)
}

fn parse_connections(input: &str) -> IResult<&str, Connections> {
    let mut result = HashMap::new();

    let (input, matches) = many1(terminated(
        tuple((
            alpha1,
            tag(" = "),
            delimited(
                char('('),
                separated_pair(alpha1, tag(", "), alpha1),
                char(')'),
            ),
        )),
        newline,
    ))(input)?;

    for (name, _, (l, r)) in matches {
        result.insert(name, (l, r));
    }

    Ok((input, result))
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Connections)> {
    all_consuming(separated_pair(parse_directions, newline, parse_connections))(input)
}

fn solve(input: &str) -> u64 {
    let (_, (directions, connections)) = parse(input).unwrap();

    let mut current: &str = "AAA";
    let mut steps: u64 = 0;

    for direction in directions.iter().cycle() {
        //dbg!((current, direction));
        steps += 1;
        let (new_l, new_r) = connections.get(current).unwrap();
        current = match direction {
            Direction::Left => new_l,
            Direction::Right => new_r,
        };

        if current == "ZZZ" {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "}
        .to_string();

        let example_solution = 2;

        let example2: String = indoc! { "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "}
        .to_string();

        let example2_solution = 6;

        assert_eq!(super::solve(&example), example_solution);
        assert_eq!(super::solve(&example2), example2_solution);
    }
}
