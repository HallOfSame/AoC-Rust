use std::fs::File;
use std::io::prelude::*;

pub fn part1(filename: &str) -> i32 {
    println!("Part 1");

    let contents = read_input(filename);

    let mut prev = contents[0];
    let mut increase_count = 0;

    for reading in contents.iter().skip(1) {
        if reading > &prev {
            increase_count += 1;
        }

        prev = *reading;
    }

    increase_count
}

pub fn part2() {}

fn read_input(filename: &str) -> Vec<i32> {
    let mut file = File::open(filename).unwrap_or_else(|_| panic!("Missing file {}", filename));

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
}
