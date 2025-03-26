use std::io::{self, Read};

/// Check if the array can be made into a zero array using the given operations
fn is_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    // Apply difference array technique for range updates
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        
        diff[l] += 1;
        if (r + 1) < nums_size {
            diff[r + 1] -= 1;
        }
    }

    // Check if the array can be zeroed
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
    
    let mut numbers = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"));
    
    // Read nums array size
    let nums_size = numbers.next().expect("Missing nums size") as usize;
    
    // Read nums array
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(numbers.next().expect("Missing nums element"));
    }
    
    // Read queries size
    let queries_size = numbers.next().expect("Missing queries size") as usize;
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let l = numbers.next().expect("Missing query start");
        let r = numbers.next().expect("Missing query end");
        queries.push(vec![l, r]);
    }
    
    // Call the function and print result
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}