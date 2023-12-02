use std::collections::HashMap;

fn main() {
    let input = include_str!("./part1.txt");
    let output = part2(input);
    print!("{}", output);
}

fn part2(input: &str) -> u32 {
    let digits: HashMap<&str, char> = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .copied()
    .collect();

    let keys = digits.keys().collect::<Vec<_>>();

    let lines = input.split("\n").map(|v| v.trim());
    let mut sum: u32 = 0;
    lines.for_each(|line| {
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;
        let mut acc = String::new();

        line.chars().for_each(|c| {
            acc = format!("{}{}", acc, c);
            if c.is_numeric() {
                if first_num.is_none() {
                    first_num = Some(c)
                }
                last_num = Some(c);
                acc = "".to_string();
            } else if let Some(&found_key) = keys.iter().find(|&&k| acc.contains(k)) {
                if first_num.is_none() {
                    first_num = Some(digits.get(found_key).unwrap().to_owned());
                }
                last_num = Some(digits.get(found_key).unwrap().to_owned());
                acc = c.to_string();
            }
        });
        sum += format!("{}{}", first_num.unwrap(), last_num.unwrap())
            .parse::<u32>()
            .unwrap();
    });
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
