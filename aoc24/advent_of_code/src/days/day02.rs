pub fn check_safe(parts: &Vec<i32>) -> bool {
    // check there is only thwo parts!
    let mut safe = true;
    let mut prev = parts[0];
    let inc = parts[1] > parts[0];
    let dec = parts[1] < parts[0];
    let eq = parts[1] == parts[0];
    if eq {
        return false;
    }
    let mut diff:i32;
    for i in 1..parts.len() {
        // step 1 check inc or dec
        if inc && !dec {
            if parts[i] < prev {
                safe = false;
                break;
            }
        } else if dec && !inc {
            if parts[i] > prev {
                safe = false;
                break;
            }
        }

        // step 2 check 1 =< diff <= 3
        diff = (prev - parts[i]).abs();
        if (diff < 1) || (3 < diff) {
            safe = false;
            break;
        }
        prev = parts[i]
    }

    return safe;
}

fn without_index_slices(vec: &[i32], index: usize) -> Vec<i32> {
    [&vec[..index], &vec[index + 1..]].concat()
}

pub fn part1(input: &str) -> String {
    // safe if:
    //The levels are either all increasing or all decreasing.
    //Any two adjacent levels differ by at least one and at most three.
    let mut safe_reports = 0;
    let mut safe = false;
    // iterate through lines!
    for line in input.lines() {
        let parts_str: Vec<&str> = line.split_whitespace().collect();
        let parts: Vec<i32> = parts_str.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        
        safe = check_safe(&parts);
        if safe {
            safe_reports += 1;
        } else {
        }
    }

  return safe_reports.to_string()

}

pub fn part2(input: &str) -> String {
    let mut safe_reports = 0;
    let mut safe = false;
    // iterate through lines!
    for line in input.lines() {
        let parts_str: Vec<&str> = line.split_whitespace().collect();
        let parts: Vec<i32> = parts_str.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        
        safe = check_safe(&parts);
        if safe {
            safe_reports += 1;
        } 
        else {
            let mut subsafe = false;
            let plen = parts.len();
            for i in 0..plen {
                let modified = without_index_slices(&parts, i); // Removes the element at index i
                subsafe |= check_safe(&modified);
            }
            if subsafe {
                safe_reports += 1;
            }

        }
    }

    return safe_reports.to_string()
}
