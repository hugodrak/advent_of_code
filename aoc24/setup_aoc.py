import os

# Configuration
PROJECT_NAME = "advent_of_code"
NUM_DAYS = 25

# Common helper functions
HELPERS_CONTENT = """
pub fn read_file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Could not read file")
}

pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}
"""

# Day template
DAY_TEMPLATE = """
pub fn part1(input: &str) -> String {
    // TODO: Implement part 1
    "N/A".to_string()
}

pub fn part2(input: &str) -> String {
    // TODO: Implement part 2
    "N/A".to_string()
}
"""

# Main template
MAIN_TEMPLATE = """
mod helpers;
mod days {{
{}
}}

fn main() {{
    let day = std::env::args().nth(1).expect("Please provide a day");
    let input_file = std::env::args().nth(2).expect("Please provide an input file path");
    let input = helpers::read_file_to_string(&input_file);

    match day.as_str() {{
{}
        _ => println!("Day not implemented"),
    }}
}}
"""

def create_project():
    # Initialize project directory
    os.makedirs(f"{PROJECT_NAME}/src/days", exist_ok=True)
    os.makedirs(f"{PROJECT_NAME}/inputs", exist_ok=True)

    # Create Cargo.toml if it doesn't exist
    cargo_toml_path = f"{PROJECT_NAME}/Cargo.toml"
    if not os.path.exists(cargo_toml_path):
        with open(cargo_toml_path, "w") as f:
            f.write(f"[package]\nname = \"{PROJECT_NAME}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n")
        print("Created Cargo.toml")

    # Create helper functions
    helpers_path = f"{PROJECT_NAME}/src/helpers.rs"
    if not os.path.exists(helpers_path):
        with open(helpers_path, "w") as f:
            f.write(HELPERS_CONTENT)
        print("Created src/helpers.rs")

    # Create day modules and input files
    mod_declarations = []
    match_arms = []
    for day in range(1, NUM_DAYS + 1):
        day_file = f"day{day:02}.rs"
        day_path = f"{PROJECT_NAME}/src/days/{day_file}"
        input_dir = f"{PROJECT_NAME}/inputs/day{day:02}"
        short_input_path = f"{input_dir}/short.txt"
        full_input_path = f"{input_dir}/full.txt"

        # Create day module
        if not os.path.exists(day_path):
            with open(day_path, "w") as f:
                f.write(DAY_TEMPLATE)
            print(f"Created src/days/{day_file}")

        # Create input directories and files
        os.makedirs(input_dir, exist_ok=True)
        if not os.path.exists(short_input_path):
            with open(short_input_path, "w") as f:
                f.write("// Short input for testing\n")
            print(f"Created {short_input_path}")
        if not os.path.exists(full_input_path):
            with open(full_input_path, "w") as f:
                f.write("// Full input for actual challenge\n")
            print(f"Created {full_input_path}")

        mod_declarations.append(f"    pub mod day{day:02};")
        match_arms.append(f"        \"{day}\" => {{\n            println!(\"Part 1: {{}}\", days::day{day:02}::part1(&input));\n            println!(\"Part 2: {{}}\", days::day{day:02}::part2(&input));\n        }}")

    # Create main.rs
    main_path = f"{PROJECT_NAME}/src/main.rs"
    if not os.path.exists(main_path):
        with open(main_path, "w") as f:
            f.write(MAIN_TEMPLATE.format("\n".join(mod_declarations), "\n".join(match_arms)))
        print("Created src/main.rs")
    else:
        print("src/main.rs already exists, not overwritten.")

# Execute script
if __name__ == "__main__":
    create_project()
    print(f"Advent of Code project '{PROJECT_NAME}' setup completed successfully!")

