use nom::branch::alt;
use nom::character::complete::newline;
use nom::combinator::complete;
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, sequence::separated_pair, IResult};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let (_, games) = complete(separated_list1(newline, parse_game))(input).unwrap();
    let mut result = 0;

    for game in games {
        if game.possible {
            result += game.id;
        }
    }

    result
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::u64(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, drawings) = separated_list1(tag("; "), parse_drawing)(input)?;

    let mut possible = true;
    for drawing in drawings {
        if drawing.0 > 12 || drawing.1 > 13 || drawing.2 > 14 {
            possible = false;
            break;
        }
    }

    Ok((input, Game { id, possible }))
}

fn parse_drawing(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let mut result = (0, 0, 0);

    let (input, pairs) = separated_list1(
        tag(", "),
        separated_pair(nom::character::complete::i32, tag(" "), parse_color),
    )(input)?;

    for (count, color) in pairs.iter() {
        match color {
            Color::Red => result.0 += count,
            Color::Green => result.1 += count,
            Color::Blue => result.2 += count,
        }
    }

    Ok((input, result))
}

#[derive(Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    use nom::combinator::value;
    alt((
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
        value(Color::Blue, tag("blue")),
    ))(input)
}

#[derive(Debug)]
struct Game {
    id: u64,
    possible: bool,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "}
        .to_string();

        let singles = [true, true, false, false, true];

        for (line, expected) in std::iter::zip(example.lines(), singles) {
            let (_, game) = super::parse_game(line).unwrap();
            dbg!(&game);
            assert_eq!(game.possible, expected);
        }

        assert_eq!(super::solve(&example), 8);
    }
}
