use std::io::{self, Read};

/// Counts valid selections based on the problem requirements
fn count_valid_selections(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    // Compute prefix sums from the left
    for i in 1..n {
        left[i] = left[i - 1] + nums[i - 1];
    }

    // Compute prefix sums from the right
    for i in 1..n {
        right[n - i - 1] = right[n - i] + nums[n - i];
    }

    // Iterate through each element to count valid selections
    for i in 0..n {
        if nums[i] != 0 {
            continue;
        }
        if left[i] == right[i] {
            res += 2;
        }
        if (left[i] - right[i]).abs() == 1 {
            res += 1;
        }
    }
    res
}

/**
 * Main Function
 * 
 * Reads input, invokes the count_valid_selections function, and prints the result.
 */
fn main() -> io::Result<()> {
    // Read all of stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input into words
    let mut words = input.split_whitespace();
    
    // Read the number of elements in the nums array
    let n: usize = words.next()
        .expect("Expected number of elements")
        .parse()
        .expect("Expected an integer for the array size");
    
    // Read the elements of the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(word) = words.next() {
            nums.push(word.parse::<i32>().expect("Expected integers in the array"));
        } else {
            panic!("Not enough integers provided");
        }
    }
    
    // Call the count_valid_selections function and store the result
    let result = count_valid_selections(&nums);
    
    // Print the result
    println!("{}", result);
    
    Ok(())
}