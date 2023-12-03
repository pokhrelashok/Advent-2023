pub fn process(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim()).collect::<Vec<&str>>();
    let last_line = lines.len();
    let mut sum: u32 = 0;

    for (line_index, line) in lines.iter().enumerate() {
        let mut number = String::new();
        for (char_index, char) in line.chars().enumerate() {
            let mut actual_index = if char_index > 0 {
                char_index - 1
            } else {
                char_index
            };
            if char.is_numeric() {
                number = format!("{number}{char}");
                if char_index == line.chars().count() - 1 {
                    actual_index = char_index;
                } else {
                    continue;
                }
            }
            if number.len() > 0 {
                let starting_index = actual_index - (number.len() - 1);
                let chars_above = if line_index != 0 {
                    lines[line_index - 1]
                        .chars()
                        .skip(if starting_index > 0 {
                            starting_index - 1
                        } else {
                            starting_index
                        })
                        .take(number.len() + 2)
                        .collect::<Vec<char>>()
                } else {
                    vec![]
                };
                let chars_below = if line_index + 1 != last_line {
                    lines[line_index + 1]
                        .chars()
                        .skip(if starting_index > 0 {
                            starting_index - 1
                        } else {
                            starting_index
                        })
                        .take(number.len() + 2)
                        .collect::<Vec<char>>()
                } else {
                    vec![]
                };
                if (starting_index > 1 && line.chars().nth(starting_index - 1).unwrap() != '.')
                    || chars_above.iter().any(|f| f != &'.' && !f.is_numeric())
                    || chars_below.iter().any(|f| f != &'.' && !f.is_numeric())
                    || (actual_index + 1 < line.chars().count()
                        && line.chars().nth(actual_index + 1).unwrap() != '.')
                {
                    sum += number.parse::<u32>().unwrap();
                }
                {}
            }
            number = "".to_string();
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.*...
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, 4361 + 58);
    }
}
