mod day01;
mod file_names;

fn main() {
    println!(
        "Day 01 Part 1: {}",
        day01::part1(&file_names::get_file_path(file_names::DAY_01_INPUT))
    );

    println!(
        "Day 01 Part 2: {}",
        day01::part2(&file_names::get_file_path(file_names::DAY_01_INPUT))
    );
}
