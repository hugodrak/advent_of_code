
pub fn read_file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Could not read file")
}

pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}
