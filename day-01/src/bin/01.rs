fn main() {
    let input = include_str!("./01.txt");
    let output = part1(input);
    print!("{}", output);
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    let mut sum: u32 = 0;
    lines.for_each(|line| {
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;
        line.chars().for_each(|c| {
            if c.is_numeric() {
                if first_num.is_none() {
                    first_num = Some(c)
                }
                last_num = Some(c)
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
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
