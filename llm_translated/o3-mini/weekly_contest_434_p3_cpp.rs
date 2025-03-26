use std::io::{self, Read};

/// Calculate the maximum frequency according to the algorithm
/// derived from the original C++ solution.
/// 
/// The logic uses three variables:
/// - f0: counts occurrences of k encountered so far.
/// - f1: an array (of fixed size 51) to maintain state for each x value.
/// - f2: variable that tracks a state across the entire array.
/// 
/// For each number x in nums:
///   f2 = max(f2, max_f1) + (if x == k then 1 else 0)
///   f1[x] = max(f1[x], f0) + 1
///   f0 = f0 + (if x == k then 1 else 0)
///   max_f1 = max(max_f1, f1[x])
/// Finally returns max(max_f1, f2).
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;
    // f1 array of fixed size 51 (indexes 0 through 50)
    let mut f1 = vec![0; 51];
    let mut max_f1 = 0;
    let mut f2 = 0;

    for &x in nums {
        // Update f2: max(current f2, max_f1) plus 1 if x equals k
        f2 = f2.max(max_f1) + if x == k { 1 } else { 0 };
        // Update f1 for the current x: max(f1[x], f0) plus one
        f1[x as usize] = f1[x as usize].max(f0) + 1;
        // If x equals k, increment f0 (the count of k seen so far)
        f0 += if x == k { 1 } else { 0 };
        // Update the overall maximum value seen in f1
        max_f1 = max_f1.max(f1[x as usize]);
    }
    max_f1.max(f2)
}

fn main() -> io::Result<()> {
    // Read the entire input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input on whitespace to extract tokens
    let mut tokens = input.split_whitespace();

    // Read the first two numbers: n (number of elements) and k (key value)
    let n: usize = tokens
        .next()
        .expect("Expected the number of elements")
        .parse()
        .expect("Failed to parse n");
    let k: i32 = tokens
        .next()
        .expect("Expected the key value k")
        .parse()
        .expect("Failed to parse k");

    // Read the next n numbers into the vector `nums`
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens
            .next()
            .expect("Expected a number in the array")
            .parse()
            .expect("Failed to parse a number");
        nums.push(num);
    }

    // Calculate the answer using the max_frequency function
    let answer = max_frequency(&nums, k);

    // Write the answer to standard output with a newline, matching the C++ output format
    println!("{}", answer);

    Ok(())
}