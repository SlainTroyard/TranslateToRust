use std::io;
use std::io::Read;

fn is_zero_array(nums: &[i32], queries: &[[i32]]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    for query in queries {
        diff[query[0] as usize] += 1;
        let r = query[1] as usize;
        if r + 1 < nums_size {
            diff[r + 1] -= 1;
        }
    }

    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read input for nums array size and elements
    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read input for queries size and elements
    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let mut queries: Vec<Vec<i32>> = Vec::new();

    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}