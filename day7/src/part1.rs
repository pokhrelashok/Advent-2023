use std::{collections::HashMap, vec};

use lazy_static::lazy_static;

static CARDS: &'static [char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
lazy_static! {
    static ref CARD_VALUE: HashMap<&'static char, usize> = {
        let mut map = HashMap::new();
        CARDS.iter().enumerate().for_each(|(ind, c)| {
            map.insert(c, 13 - ind);
        });
        map
    };
}

pub fn hand_value(hand: &str, bet: usize) -> Vec<usize> {
    let mut char_map: HashMap<char, usize> = HashMap::new();
    let mut total_value = hand
        .chars()
        .map(|c| {
            let val = CARD_VALUE.get(&c).unwrap().clone();
            char_map.entry(c).and_modify(|f| *f += 1).or_insert(1);
            val
        })
        .collect::<Vec<usize>>();
    let of_type = char_map.values().fold(0, |total, &curr| {
        let mut addition = 0;
        if curr == 2 {
            addition = 1;
        } else if curr == 3 {
            addition = 3;
        } else if curr == 4 {
            addition = 5;
        } else if curr == 5 {
            addition = 6;
        }
        total + addition
    });
    total_value.insert(0, of_type.to_owned());
    total_value.push(bet);
    total_value
}

pub fn process(input: &str) -> usize {
    let mut lines = input.split("\n").map(|v| v.trim()).collect::<Vec<&str>>();
    lines = lines.iter().map(|line| line.trim()).collect();

    let mut result = lines.iter().fold(vec![], |mut all, line| {
        let inputs = line.split(" ").collect::<Vec<&str>>();
        let (hand, bet) = (inputs[0], inputs[1].parse::<usize>().unwrap());
        let calculated_value = hand_value(hand, bet);
        all.push(calculated_value);
        all
    });
    // Sorting each sub-list based on the first element, then second, and so on
    result.sort_by(|a, b| {
        a.iter()
            .zip(b.iter())
            .find(|(x, y)| x != y)
            .map_or(std::cmp::Ordering::Equal, |(x, y)| x.cmp(y))
    });
    result
        .iter()
        .enumerate()
        .fold(0 as usize, |mut total, (index, curr)| {
            total += (index + 1) * curr[curr.len() - 1];
            total
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}
