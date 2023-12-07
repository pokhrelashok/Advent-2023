use day5::part2::process;

fn main() {
    let input = include_str!("../part2.txt");
    let output = process(input);
    print!("{}", output);
}
