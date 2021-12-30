use std::fs::File;
use std::io::prelude::*;

pub fn part1(file_name: &str) -> i32 {
    println!("Part 1");

    let contents = read_input(file_name);

    return get_increased_value_count(contents);
}

pub fn part2(file_name: &str) -> i32 {
    let contents = read_input(file_name);

    let mut sums = Vec::new();

    for i in 2..contents.len() {
        let sum = contents[i - 2] + contents[i - 1] + contents[i];

        sums.push(sum);
    }

    return get_increased_value_count(sums);
}

fn get_increased_value_count(value_vec: Vec<i32>) -> i32 {
    let mut prev = value_vec[0];
    let mut increase_count = 0;

    for reading in value_vec.iter().skip(1) {
        if reading > &prev {
            increase_count += 1;
        }

        prev = *reading;
    }

    increase_count
}

fn read_input(file_name: &str) -> Vec<i32> {
    let mut file = File::open(file_name).unwrap_or_else(|_| panic!("Missing file {}", file_name));

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let split_contents = contents.split("\r\n");

    let mut result = Vec::new();

    for line in split_contents {
        result.push(
            line.parse::<i32>()
                .unwrap_or_else(|_| panic!("Value {} was not a valid int", line)),
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn part_one_example_works() {
        test_part_one("input/day01-example01.txt", 7);
    }

    #[test]
    fn part_one_works() {
        test_part_one("input/day01.txt", 1288);
    }

    fn test_part_one(file_name: &str, expected_result: i32) {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push(file_name);

        let part_one_result = part1(file_path.to_str().unwrap());

        assert_eq!(part_one_result, expected_result);
    }

    fn test_part_two(file_name: &str, expected_result: i32) {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push(file_name);

        let part_two_result = part2(file_path.to_str().unwrap());

        assert_eq!(part_two_result, expected_result);
    }

    #[test]
    fn part_two_example_works() {
        test_part_two("input/day01-example01.txt", 5);
    }

    #[test]
    fn part_two_works() {
        test_part_two("input/day01.txt", 1311);
    }
}
