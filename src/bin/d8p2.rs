use nom::combinator::all_consuming;
use nom::{
    character::complete::alphanumeric1,
    sequence::{separated_pair, tuple},
};
use std::{collections::HashMap, io::Read};

use lcmx::lcmx;
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
            alphanumeric1,
            tag(" = "),
            delimited(
                char('('),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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

    let cycle_lengths: Vec<u64> = connections
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .map(|x| find_cycle_length(x, &directions, &connections))
        .collect();

    lcmx(&cycle_lengths[..]).unwrap()
}

fn find_cycle_length(
    starting_position: &str,
    directions: &[Direction],
    connections: &Connections,
) -> u64 {
    let mut position = starting_position;
    let mut steps = 0;

    for direction in directions.iter().cycle() {
        //dbg!((current, direction));
        steps += 1;
        let (new_l, new_r) = connections.get(position).unwrap();
        position = match direction {
            Direction::Left => new_l,
            Direction::Right => new_r,
        };

        if position.ends_with('Z') {
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
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "}
        .to_string();

        let example_solution = 6;

        assert_eq!(super::solve(&example), example_solution);
    }
}
