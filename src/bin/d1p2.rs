use aho_corasick::AhoCorasick;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(input);

    println!("{result}");
}

fn solve(input: String) -> u64 {
    let mut sum: u64 = 0;

    let patterns = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    for dirty_line in input.lines() {
        let mut matches = ac.find_overlapping_iter(dirty_line);

        let first_match = matches.next().unwrap();
        let first = first_match.pattern().as_u64() % 10;
        let last = matches.last().unwrap_or(first_match).pattern().as_u64() % 10;

        sum += 10 * first + last;
    }

    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "}
        .to_string();

        let singles = [29, 83, 13, 24, 42, 14, 76];

        for (line, expected) in std::iter::zip(example.lines(), singles) {
            assert_eq!(super::solve(line.to_string()), expected);
        }

        assert_eq!(super::solve(example), 281);
    }
}
