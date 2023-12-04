pub fn process(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    let mut sum: u32 = 0;
    lines.for_each(|line| {
        let input = line.split(": ").last().unwrap();
        let winning_cards: Vec<&str> = (input.split(" | ").next().unwrap()).split(" ").collect();
        let i_have: Vec<&str> = (input.split(" | ").last().unwrap()).split(" ").collect();

        let valid_winning_cards: Vec<u32> = winning_cards
            .iter()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        let valid_i_have: Vec<u32> = i_have
            .iter()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();

        let mut winning_numbers = 0;

        valid_i_have.iter().for_each(|card| {
            if valid_winning_cards.contains(card) {
                winning_numbers = if winning_numbers == 0 {
                    1
                } else {
                    winning_numbers * 2
                };
            }
        });
        println!("{:?} {:?}", valid_winning_cards, valid_i_have);
        sum = sum + winning_numbers
    });
    return sum;
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
        assert_eq!(result, 13);
    }
}
