use day7::part1::process;
fn main() {
    let input = include_str!("../part1.txt");
    let output = process(input);
    print!("{}", output);
}
