use regex::Regex;

pub fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // mul(X,Y) X, Y is 1-3 digit

    let mut mulsum = 0;
    for caps in re.captures_iter(input) {
        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
            if let (Ok(x), Ok(y)) = (x.as_str().parse::<i32>(), y.as_str().parse::<i32>()) {
                // println!("{}, {}", x, y);
                mulsum += x*y;
            }
        }        
    }
    return mulsum.to_string()
}

pub fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();

    // mul(X,Y) X, Y is 1-3 digit

    let mut mulsum = 0;
    let mut store: bool = true;
    for caps in re.captures_iter(input) {
        if let Some(kw1) = caps.get(3) {
            if kw1.as_str() == "do" {
                store = true;
                continue;
            }
        }
        
        if let Some(kw2) = caps.get(4) {
            if kw2.as_str() == "don't" {
                store = false;
                continue;
            }
        }

        if store {
            if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
                if let (Ok(x), Ok(y)) = (x.as_str().parse::<i32>(), y.as_str().parse::<i32>()) {
                    // println!("{}, {}", x, y);
                    mulsum += x*y;
                }
            }
        } 
    }
    return mulsum.to_string()
}
