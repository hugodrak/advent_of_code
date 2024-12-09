use regex::Regex;

pub fn get_matches(lines: &Vec<String>) -> i32 {
    let re1 = Regex::new(r"SAMX").unwrap();
    let re2 = Regex::new(r"XMAS").unwrap();
    let mut count: i32 = 0;
    for line in lines {
        for _caps in re1.captures_iter(line) {
            count += 1;
        }
        for _caps in re2.captures_iter(line) {
            count += 1;
        }
    }
    return count;
}

pub fn diagonals(input: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let n = lines.len(); // Assuming square input
    let mut tlbr: Vec<String> = vec![String::new(); 2 * n - 1]; // Top-left to bottom-right diagonals
    let mut trbl: Vec<String> = vec![String::new(); 2 * n - 1]; // Top-right to bottom-left diagonals

    // Iterate through each character by its row and column
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            // Characters on the same top-left to bottom-right diagonal share the same index `i - j + n - 1`
            let tlbr_index = i + j; // Diagonal index for top-left to bottom-right
            tlbr[tlbr_index].push(ch);

            // Characters on the same top-right to bottom-left diagonal share the same index `i + j`
            let trbl_index = (n - 1 - i) + j; // Diagonal index for top-right to bottom-left
            trbl[trbl_index].push(ch);
        }
    }

    (tlbr, trbl)
}

pub fn part1(input: &str) -> String {
    // Split input into lines and collect into a vector
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    // 1. horizontal lines -> works
    let sum1 = get_matches(&lines);
  

    // 2. vertical lines
    let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut columns: Vec<String> = vec![String::new(); num_cols];
    println!("COLS: {}", num_cols);

    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            columns[i].push(ch);
        }
    }

    let sum2 = get_matches(&columns);


    let (tlbr, trbl) = diagonals(input);
    // 3. tl to br lines
    let sum3 = get_matches(&tlbr);
    // 4. tr to bl lines
    let sum4 = get_matches(&trbl);

    println!("{}, {}, {}, {}", sum1, sum2, sum3, sum4);


    return (sum1+sum2+sum3+sum4).to_string()
}

pub fn part2(_input: &str) -> String {
    // TODO: Implement part 2
    "N/A".to_string()
}
