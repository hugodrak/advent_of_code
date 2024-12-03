use std::collections::HashMap;

pub fn part1(input: &str) -> String {
  // create two vectors to store right and left!
  let mut left_nums = Vec::new();
  let mut right_nums = Vec::new();

  // iterate through lines!
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    // check there is only thwo parts!
    if parts.len() == 2 {
        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            left_nums.push(left);
            right_nums.push(right);
        }
    }    
  }


  // sort the vectors:
  left_nums.sort();
  right_nums.sort();

  // check same lenght vecs:
  let mut dist_sum = 0;
  let vec_len = left_nums.len();
  if left_nums.len() == right_nums.len() {
        for i in 0..vec_len {
            dist_sum += (left_nums[i] - right_nums[i]).abs();
        }
  }



  return dist_sum.to_string();

}

pub fn part2(input: &str) -> String {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    // create two vectors to store right and left!
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();
  
    // iterate through lines!
    for line in input.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      // check there is only thwo parts!
      if parts.len() == 2 {
          if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
              left_nums.push(left);
              right_nums.push(right);
          }
      }    
    }

    // build hashmap with right counts
    for r in right_nums {
        if counter.contains_key(&r) {
            *counter.entry(r).or_insert(0) += 1;
        } else {
            counter.insert(r, 1);
        }
    }

    // iterate through left and find match in right
    let mut sum = 0;
    for l in left_nums {
        if counter.contains_key(&l) {
            if let Some(&value) = counter.get(&l) {
                sum += l * value;
            }
        }        

    }
    
    return sum.to_string();
}
