fn main() {
    let input = include_str!("./01.txt");
    let output = part1(input);
    print!("{}", output);
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("");
        assert_eq!(result, 0);
    }
}
