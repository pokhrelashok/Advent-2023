use std::collections::HashMap;

fn main() {
    let input = include_str!("./02.txt");
    let output = part2(input);
    print!("{}", output);
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("");
        assert_eq!(result, 0);
    }
}
