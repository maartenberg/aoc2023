use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let mut gears = std::collections::HashSet::<(usize, usize)>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '*' {
                gears.insert((x, y));
            }
        }
    }

    let mut numbers = std::collections::HashMap::<(usize, usize), (usize, u64)>::new();
    let mut number_number = 0;

    for (y, line) in input.lines().enumerate() {
        let mut number: u64 = 0;
        let mut number_start: Option<usize> = None;

        for (x, c) in line.char_indices() {
            match c.to_digit(10) {
                None => match number_start {
                    None => {}
                    Some(xs) => {
                        for u in xs..x {
                            numbers.insert((u, y), (number_number, number));
                        }
                        number_number += 1;

                        number = 0;
                        number_start = None;
                    }
                },
                Some(d) => {
                    number *= 10;
                    number += d as u64;
                    if number_start.is_none() {
                        number_start = Some(x);
                    }
                }
            }
        }

        if let Some(xs) = number_start {
            let x = line.chars().count();
            for u in xs..x {
                numbers.insert((u, y), (number_number, number));
            }
            number_number += 1;
        }
    }

    let mut result: u64 = 0;

    for gear in gears.iter() {
        let (gear_x, gear_y) = gear;

        let mut adjacent = HashSet::<(usize, u64)>::new();

        for y in gear_y.saturating_sub(1)..=gear_y + 1 {
            for x in gear_x.saturating_sub(1)..=gear_x + 1 {
                if let Some(num) = numbers.get(&(x, y)) {
                    adjacent.insert(*num);
                }
            }
        }

        if adjacent.len() != 2 {
            continue;
        }

        result += adjacent.iter().map(|(_, v)| v).product::<u64>();
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

        let example_solution = 467835;

        assert_eq!(super::solve(&example), example_solution);
    }
}
