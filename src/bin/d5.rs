use nom::{combinator::all_consuming, sequence::separated_pair};
use std::io::Read;
use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1, u64},
    multi::separated_list1,
    sequence::{terminated, tuple},
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

type TranslationMap = Vec<(Range<u64>, u64)>;

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<TranslationMap>)> {
    all_consuming(separated_pair(
        parse_seeds,
        newline,
        separated_list1(newline, parse_map),
    ))(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;

    terminated(separated_list1(space1, u64), newline)(input)
}

fn parse_map(input: &str) -> IResult<&str, TranslationMap> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:"), newline))(input)?;

    let (input, maplines) = terminated(
        separated_list1(newline, tuple((u64, space1, u64, space1, u64))),
        newline,
    )(input)?;

    let mut result = Vec::new();

    for (to_start, _, from_start, _, range) in maplines {
        result.push((from_start..from_start + range, to_start));
    }

    Ok((input, result))
}

fn solve(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse(input).unwrap();

    let mut values = Vec::from(seeds);

    for map in maps {
        for v in values.iter_mut() {
            for (range, new_set) in map.iter() {
                if !range.contains(v) {
                    continue;
                }

                *v = new_set + (*v - range.start);
                break;
            }
        }
    }

    *values.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "}
        .to_string();

        let example_solution = 35;

        assert_eq!(super::solve(&example), example_solution);
    }
}
