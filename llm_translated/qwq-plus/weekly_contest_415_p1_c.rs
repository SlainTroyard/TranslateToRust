use std::io;

/// Translates the C code to Rust, preserving the algorithm and input/output logic.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let array_size = n + 2;
    let nums: Vec<i32> = parts
        .map(|s| s.parse().unwrap())
        .take(array_size)
        .collect();
    let result = get_sneaky_numbers(&nums);
    println!("{} {}", result[0], result[1]);
}

/// Finds the first two "sneaky" numbers (duplicates) in the array.
///
/// # Arguments
///
/// * `nums` - Slice of integers to search.
///
/// # Returns
///
/// A vector of two integers, where the first elements are the first two duplicates found,
/// and the rest are initialized to zero if not found.
fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2];
    let mut count = 0;
    for (i, &num) in nums.iter().enumerate() {
        // Check if there's a duplicate after the current index
        if nums[i + 1..].iter().any(|&x| x == num) {
            result[count] = num;
            count += 1;
            if count == 2 {
                break;
            }
        }
    }
    result
}