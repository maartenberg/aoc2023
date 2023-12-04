use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::{space1, u64};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashSet;
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
    let (_, cards) =
        all_consuming(terminated(separated_list1(newline, parse_card), newline))(input).unwrap();

    let mut card_counts: Vec<u64> = Vec::new();
    card_counts.resize(cards.len(), 1);

    for (card_num, card_value) in cards.iter().enumerate() {
        let current_count = card_counts[card_num];

        for other_card in (card_num + 1)..=(card_num + (*card_value as usize)) {
            card_counts[other_card] += current_count;
        }
    }

    card_counts.iter().sum()
}

fn parse_card(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;

    let (input, (winners, haves)) = separated_pair(
        separated_list1(space1, u64),
        nom::sequence::tuple((space1, tag("|"), space1)),
        separated_list1(space1, u64),
    )(input)?;

    let winset: HashSet<u64> = HashSet::from_iter(winners);
    let mut wincount = 0;

    for have in haves {
        if winset.contains(&have) {
            wincount += 1;
        }
    }

    Ok((input, wincount))
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "}
        .to_string();

        let singles = [4, 2, 2, 1, 0, 0];

        for (line, expected) in zip(example.lines(), singles) {
            assert_eq!(super::parse_card(line).unwrap().1, expected);
        }

        let example_solution = 30;

        assert_eq!(super::solve(&example), example_solution);
    }
}
