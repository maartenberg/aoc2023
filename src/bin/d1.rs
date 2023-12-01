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

    for line in input.lines() {
        let mut digits = line.chars().filter(|c| c.is_digit(10));

        let first = digits.next().map(|c| c.to_digit(10).unwrap()).unwrap();
        let last = digits
            .last()
            .map(|c| c.to_digit(10).unwrap())
            .unwrap_or(first);

        sum += (10 * first + last) as u64;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_indoc() {
        let expected = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";

        let actual = indoc! { "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "}
        .to_string();

        assert_eq!(super::solve(example), 142);
    }
}
