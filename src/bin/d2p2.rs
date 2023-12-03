use std::cmp::max;
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
        result += game.power;
    }

    result
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _id) = nom::character::complete::u64(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, drawings) = separated_list1(tag("; "), parse_drawing)(input)?;

    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for drawing in drawings {
        min_red = max(min_red, drawing.0);
        min_green = max(min_green, drawing.1);
        min_blue = max(min_blue, drawing.2);
    }

    let power = min_red * min_green * min_blue;

    Ok((input, Game { power }))
}

fn parse_drawing(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let mut result = (0, 0, 0);

    let (input, pairs) = separated_list1(
        tag(", "),
        separated_pair(nom::character::complete::u64, tag(" "), parse_color),
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
    power: u64,
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

        let singles = [48, 12, 1560, 630, 36];

        for (line, expected) in std::iter::zip(example.lines(), singles) {
            let (_, game) = super::parse_game(line).unwrap();
            dbg!(&game);
            assert_eq!(game.power, expected);
        }

        assert_eq!(super::solve(&example), 2286);
    }
}
