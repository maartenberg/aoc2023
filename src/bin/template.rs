use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read fail");

    let result = solve(&input);

    println!("{result}");
}

fn solve(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let example: String = indoc! { "
            examplehere
        "}
        .to_string();

        let example_solution = 0;

        assert_eq!(super::solve(&example), example_solution);
    }
}
