
mod helpers;
mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

fn main() {
    let day = std::env::args().nth(1).expect("Please provide a day");
    let input_file = std::env::args().nth(2).expect("Please provide an input file path");
    let input = helpers::read_file_to_string(&input_file);

    match day.as_str() {
        "1" => {
            println!("Part 1: {}", days::day01::part1(&input));
            println!("Part 2: {}", days::day01::part2(&input));
        }
        "2" => {
            println!("Part 1: {}", days::day02::part1(&input));
            println!("Part 2: {}", days::day02::part2(&input));
        }
        "3" => {
            println!("Part 1: {}", days::day03::part1(&input));
            println!("Part 2: {}", days::day03::part2(&input));
        }
        "4" => {
            println!("Part 1: {}", days::day04::part1(&input));
            println!("Part 2: {}", days::day04::part2(&input));
        }
        "5" => {
            println!("Part 1: {}", days::day05::part1(&input));
            println!("Part 2: {}", days::day05::part2(&input));
        }
        "6" => {
            println!("Part 1: {}", days::day06::part1(&input));
            println!("Part 2: {}", days::day06::part2(&input));
        }
        "7" => {
            println!("Part 1: {}", days::day07::part1(&input));
            println!("Part 2: {}", days::day07::part2(&input));
        }
        "8" => {
            println!("Part 1: {}", days::day08::part1(&input));
            println!("Part 2: {}", days::day08::part2(&input));
        }
        "9" => {
            println!("Part 1: {}", days::day09::part1(&input));
            println!("Part 2: {}", days::day09::part2(&input));
        }
        "10" => {
            println!("Part 1: {}", days::day10::part1(&input));
            println!("Part 2: {}", days::day10::part2(&input));
        }
        "11" => {
            println!("Part 1: {}", days::day11::part1(&input));
            println!("Part 2: {}", days::day11::part2(&input));
        }
        "12" => {
            println!("Part 1: {}", days::day12::part1(&input));
            println!("Part 2: {}", days::day12::part2(&input));
        }
        "13" => {
            println!("Part 1: {}", days::day13::part1(&input));
            println!("Part 2: {}", days::day13::part2(&input));
        }
        "14" => {
            println!("Part 1: {}", days::day14::part1(&input));
            println!("Part 2: {}", days::day14::part2(&input));
        }
        "15" => {
            println!("Part 1: {}", days::day15::part1(&input));
            println!("Part 2: {}", days::day15::part2(&input));
        }
        "16" => {
            println!("Part 1: {}", days::day16::part1(&input));
            println!("Part 2: {}", days::day16::part2(&input));
        }
        "17" => {
            println!("Part 1: {}", days::day17::part1(&input));
            println!("Part 2: {}", days::day17::part2(&input));
        }
        "18" => {
            println!("Part 1: {}", days::day18::part1(&input));
            println!("Part 2: {}", days::day18::part2(&input));
        }
        "19" => {
            println!("Part 1: {}", days::day19::part1(&input));
            println!("Part 2: {}", days::day19::part2(&input));
        }
        "20" => {
            println!("Part 1: {}", days::day20::part1(&input));
            println!("Part 2: {}", days::day20::part2(&input));
        }
        "21" => {
            println!("Part 1: {}", days::day21::part1(&input));
            println!("Part 2: {}", days::day21::part2(&input));
        }
        "22" => {
            println!("Part 1: {}", days::day22::part1(&input));
            println!("Part 2: {}", days::day22::part2(&input));
        }
        "23" => {
            println!("Part 1: {}", days::day23::part1(&input));
            println!("Part 2: {}", days::day23::part2(&input));
        }
        "24" => {
            println!("Part 1: {}", days::day24::part1(&input));
            println!("Part 2: {}", days::day24::part2(&input));
        }
        "25" => {
            println!("Part 1: {}", days::day25::part1(&input));
            println!("Part 2: {}", days::day25::part2(&input));
        }
        _ => println!("Day not implemented"),
    }
}
