mod day12;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day12::solve1();
}
