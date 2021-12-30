use std::path::PathBuf;

#[cfg(test)]
pub const DAY_01_EXAMPLE_INPUT: &str = "day01-example01.txt";

pub const DAY_01_INPUT: &str = "day01.txt";

pub fn get_file_path(file_name: &str) -> String {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("input");
    file_path.push(file_name);

    return file_path.into_os_string().into_string().unwrap();
}
