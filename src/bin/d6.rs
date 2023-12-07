use nom::{
    character::complete::{newline, u128},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::io::Read;

use nom::{bytes::complete::tag, character::complete::space1, sequence::tuple};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

struct Race {
    race_duration: u128,
    distance_to_beat: u128,
}

impl Race {
    fn possibilities(&self) -> u128 {
        let (t, d): (f64, f64) = (self.race_duration as f64, self.distance_to_beat as f64 + 0.00001);
        let min = 0.5 * (t - (t.powi(2) - 4.0 * d).sqrt());
        let first = min.floor() as u128;
        let last = (t - min).floor() as u128;

        let result = last - first;

        dbg!((t, d, min, first, last, result));

        result
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;

    let (input, race_times) = terminated(separated_list1(space1, u128), newline)(input)?;
    let (input, _) = tuple((tag("Distance:"), space1))(input)?;

    let (input, race_distances) =
        all_consuming(terminated(separated_list1(space1, u128), newline))(input)?;

    let races = race_times
        .iter()
        .zip(race_distances)
        .map(|(race_duration, distance_to_beat)| Race {
            race_duration: *race_duration,
            distance_to_beat,
        })
        .collect();

    Ok((input, races))
}

fn solve(input: &str) -> u128 {
    let (_, races) = parse(input).unwrap();

    races.iter().map(|r| r.possibilities()).product()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            Time:      7  15   30
            Distance:  9  40  200
        "}
        .to_string();

        let races = super::parse(&example).unwrap().1;

        let singles = [4, 8, 9];

        for (race, expected) in races.iter().zip(singles) {
            assert_eq!(race.possibilities(), expected);
        }

        let example_solution = 288;

        assert_eq!(super::solve(&example), example_solution);
    }
}
