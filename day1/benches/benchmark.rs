use day1::part1;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../src/part1.txt")));
}
#[divan::bench]
fn part2() {
    part1::process(divan::black_box(include_str!("../src/part2.txt")));
}
