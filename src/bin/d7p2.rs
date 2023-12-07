use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt,
    character::complete::{char, newline, u64},
    combinator::{all_consuming, value},
    multi::many1,
    sequence::{separated_pair, terminated, tuple},
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

fn solve(input: &str) -> u64 {
    let (_, mut input) = parse(input).unwrap();

    input.sort();
    input.reverse();

    input
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * *bid)
        .sum()
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Hand(Card, Card, Card, Card, Card);

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct TypedHand(HandType, Hand);

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        use HandType::*;

        let cards = vec![self.0, self.1, self.2, self.3, self.4];

        let mut counter: HashMap<Card, u8> = HashMap::new();
        let mut jokers = 0;
        for card in cards {
            if card == Card::Joker {
                jokers += 1;
            } else {
                *counter.entry(card).or_default() += 1;
            }
        }

        let mut counts: Vec<u8> = counter.drain().map(|(_, v)| v).collect();
        counts.sort();
        counts.reverse();

        if let Some(x) = counts.get_mut(0) {
            *x += jokers;
        } else {
            counts.push(jokers);
        }

        match counts[..] {
            [5] => FiveOfAKind,
            [4, 1] => FourOfAKind,
            [3, 2] => FullHouse,
            [3, 1, 1] => ThreeOfAKind,
            [2, 2, 1] => TwoPair,
            [2, 1, 1, 1] => OnePair,
            [1, 1, 1, 1, 1] => HighCard,
            _ => unreachable!("{:?}", counts),
        }
    }

    fn typed(self) -> TypedHand {
        TypedHand(self.hand_type(), self)
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    use Card::*;
    alt((
        value(Ace, char('A')),
        value(King, char('K')),
        value(Queen, char('Q')),
        value(Joker, char('J')),
        value(Ten, char('T')),
        value(Nine, char('9')),
        value(Eight, char('8')),
        value(Seven, char('7')),
        value(Six, char('6')),
        value(Five, char('5')),
        value(Four, char('4')),
        value(Three, char('3')),
        value(Two, char('2')),
    ))(input)
}

fn parse_hand(input: &str) -> IResult<&str, TypedHand> {
    let (input, (a, b, c, d, e)) =
        tuple((parse_card, parse_card, parse_card, parse_card, parse_card))(input)?;
    Ok((input, Hand(a, b, c, d, e).typed()))
}

fn parse(input: &str) -> IResult<&str, Vec<(TypedHand, u64)>> {
    all_consuming(many1(terminated(
        separated_pair(parse_hand, char(' '), u64),
        newline,
    )))(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_parse() {
        use super::HandType::*;
        let examples = [
            ("AAAAA", FiveOfAKind),
            ("AA8AA", FourOfAKind),
            ("23332", FullHouse),
            ("TTT98", ThreeOfAKind),
            ("23432", TwoPair),
            ("A23A4", OnePair),
            ("23456", HighCard),
        ];

        for (hand, handtype) in examples {
            assert_eq!(super::parse_hand(hand).unwrap().1 .0, handtype);
        }
    }

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "}
        .to_string();

        let example_solution = 6440;

        let mut parsed = super::parse(&example).unwrap().1;
        parsed.sort();
        parsed.reverse();
        dbg!(parsed);

        assert_eq!(super::solve(&example), example_solution);
    }
}
