use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

fn solve(input: &str) -> u32 {
    let mut result = 0;

    let mut symbols = std::collections::HashSet::<(usize, usize)>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if !c.is_digit(10) && c != '.' {
                symbols.insert((x, y));
            }
        }
    }

    for (y, line) in input.lines().enumerate() {
        let mut number = 0;
        let mut number_start: Option<usize> = None;

        for (x, c) in line.char_indices() {
            match c.to_digit(10) {
                None => match number_start {
                    None => {}
                    Some(xs) => {
                        'outer: for u in (xs.saturating_sub(1))..=x {
                            for v in y.saturating_sub(1)..=y.saturating_add(1) {
                                if symbols.contains(&(u, v)) {
                                    result += number;
                                    break 'outer;
                                }
                            }
                        }

                        number = 0;
                        number_start = None;
                    }
                },
                Some(d) => {
                    number *= 10;
                    number += d;
                    if number_start.is_none() {
                        number_start = Some(x);
                    }
                }
            }
        }

        if let Some(xs) = number_start {
            let x = line.chars().count();
            'outer: for u in (xs.saturating_sub(1))..=x {
                for v in y.saturating_sub(1)..=y.saturating_add(1) {
                    if symbols.contains(&(u, v)) {
                        result += number;
                        break 'outer;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "}
        .to_string();

        let example_solution = 4361;

        assert_eq!(super::solve(&example), example_solution);
    }
}
