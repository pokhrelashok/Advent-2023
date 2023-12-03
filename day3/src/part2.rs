use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim()).collect::<Vec<&str>>();
    let mut sum: u32 = 0;
    let mut tree: HashMap<String, u32> = HashMap::new();

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
                let last_index = starting_index + number.len() - 1;

                for i in starting_index..last_index + 1 {
                    tree.insert(
                        format!("{}-{}", line_index, i,),
                        number.clone().parse::<u32>().unwrap(),
                    );
                }
            }
            number = "".to_string();
        }
    }

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char != '*' {
                continue;
            }
            let mut ratios: Vec<u32> = vec![];

            if char_index > 0 {
                let left_key = format!("{}-{}", line_index, char_index - 1);
                if tree.contains_key(&left_key) {
                    ratios.push(tree.get(&left_key).unwrap().to_owned())
                }
            }

            if char_index < line.chars().count() - 1 {
                let right_key = format!("{}-{}", line_index, char_index + 1);
                if tree.contains_key(&right_key) {
                    ratios.push(tree.get(&right_key).unwrap().to_owned())
                }
            }

            let (top_indices, bottom_indices) =
                get_vertical_indices(char_index, line_index, line.chars().count(), lines.len());

            if line_index > 0 {
                if tree.contains_key(&format!("{}-{}", line_index - 1, char_index)) {
                    ratios.push(
                        tree.get(&format!("{}-{}", line_index - 1, char_index))
                            .unwrap()
                            .to_owned(),
                    )
                } else {
                    top_indices.iter().for_each(|index| {
                        if tree.contains_key(index) {
                            ratios.push(tree.get(index).unwrap().to_owned())
                        }
                    })
                }
            }

            if line_index < lines.len() - 1 {
                if tree.contains_key(&format!("{}-{}", line_index + 1, char_index)) {
                    ratios.push(
                        tree.get(&format!("{}-{}", line_index + 1, char_index))
                            .unwrap()
                            .to_owned(),
                    )
                } else {
                    bottom_indices.iter().for_each(|index| {
                        if tree.contains_key(index) {
                            ratios.push(tree.get(index).unwrap().to_owned())
                        }
                    })
                }
            }

            if ratios.len() == 2 {
                sum = sum + ratios.iter().product::<u32>();
            }
        }
    }

    return sum;
}

fn get_vertical_indices(
    index: usize,
    line: usize,
    row_length: usize,
    column_length: usize,
) -> (Vec<String>, Vec<String>) {
    let mut top_indices: Vec<String> = vec![];
    let mut bottom_indices: Vec<String> = vec![];
    if line != 0 {
        if index != 0 {
            top_indices.push(format!("{}-{}", line - 1, index - 1));
        }
        if index != row_length - 1 {
            top_indices.push(format!("{}-{}", line - 1, index + 1));
        }
    }
    if line != column_length - 1 {
        if index != 0 {
            bottom_indices.push(format!("{}-{}", line + 1, index - 1));
        }
        if index != row_length - 1 {
            bottom_indices.push(format!("{}-{}", line + 1, index + 1));
        }
    }
    (top_indices, bottom_indices)
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
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, 467835);
    }
}
