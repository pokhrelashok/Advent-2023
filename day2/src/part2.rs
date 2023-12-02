use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    let mut sum: u32 = 0;
    lines.for_each(|line| {
        let split = line.split(":");
        let inputs = split.last().unwrap();
        let mut min_req: HashMap<&str, u32> = HashMap::new();
        min_req.insert("red", 0);
        min_req.insert("blue", 0);
        min_req.insert("green", 0);

        for game in inputs.split(";") {
            game.split(",").map(|f| f.trim()).for_each(|ball| {
                let color = COLORS.iter().find(|&&c| ball.contains(c)).unwrap();
                let count = ball.replace(color, "").trim().parse::<u32>().unwrap();

                if count > min_req.get(color).unwrap().to_owned() {
                    min_req.insert(color, count);
                }
            });
        }
        let power = min_req.get("red").unwrap()
            * min_req.get("blue").unwrap()
            * min_req.get("green").unwrap();
        sum += power;
    });
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
