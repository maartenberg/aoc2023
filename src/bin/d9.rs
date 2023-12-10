use nom::IResult;
use std::io::Read;

use nom::{character::complete::i64, combinator::all_consuming};
use nom::{
    character::complete::{char, newline},
    multi::{many1, separated_list1},
    sequence::terminated,
};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

fn solve(input: &str) -> i64 {
    let (_, inputs) = parse(input).unwrap();

    inputs.iter().map(solve_single).sum()
}

fn solve_single(numbers: &Vec<i64>) -> i64 {
    let mut lasts: Vec<i64> = Vec::new();
    lasts.push(numbers.last().cloned().unwrap());

    let mut changingnumbers = numbers.clone();

    loop {
        changingnumbers = changingnumbers
            .iter()
            .zip(changingnumbers.iter().skip(1))
            .map(|(l, r)| r - l)
            .collect();

        lasts.push(changingnumbers.last().cloned().unwrap());

        if changingnumbers.iter().all(|&x| x == 0) {
            break;
        }
    }

    //dbg!(&lasts);

    lasts.iter().sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    all_consuming(many1(terminated(separated_list1(char(' '), i64), newline)))(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "}
        .to_string();

        let example_solution = 114;

        assert_eq!(super::solve(&example), example_solution);
    }
}
