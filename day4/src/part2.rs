use std::collections::HashMap;

pub fn process(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").map(|v| v.trim()).collect();
    let mut scratch_count: HashMap<usize, usize> = HashMap::new();
    lines.iter().enumerate().for_each(|(index, line)| {
        let input = line.split(": ").last().unwrap();
        let winning_cards: Vec<&str> = (input.split(" | ").next().unwrap()).split(" ").collect();
        let i_have: Vec<&str> = (input.split(" | ").last().unwrap()).split(" ").collect();

        let valid_winning_cards: Vec<usize> = winning_cards
            .iter()
            .filter_map(|f| f.parse::<usize>().ok())
            .collect();
        let valid_i_have: Vec<usize> = i_have
            .iter()
            .filter_map(|f| f.parse::<usize>().ok())
            .collect();

        let mut matching_numbers = 0;
        let times = scratch_count
            .get(&index)
            .or_else(|| Some(&0))
            .unwrap()
            .to_owned();

        valid_i_have.iter().for_each(|card| {
            if valid_winning_cards.contains(card) {
                matching_numbers += 1;
            }
        });

        for i in index + 1..index + 1 + matching_numbers {
            scratch_count
                .entry(i)
                .and_modify(|value| {
                    *value += (times + 1);
                })
                .or_insert((times + 1));
        }
    });
    let sum: usize = scratch_count.values().sum();
    return sum + lines.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
