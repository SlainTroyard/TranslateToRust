use std::io;

/// Constructs a transformed array based on the given rules.
///
/// For each element in the input array:
/// - If the element is 0, it remains 0 in the result.
/// - Otherwise, the element's value (steps) determines the target index via (i + steps) % nums_size.
/// - The target index is adjusted to be non-negative if necessary.
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let n_i32 = n as i32;
    let mut result = Vec::with_capacity(n);

    for (i, &num) in nums.iter().enumerate() {
        if num == 0 {
            result.push(0);
            continue;
        }

        let steps = num;
        let mut target_index = (i as i32 + steps) % n_i32;
        if target_index < 0 {
            target_index += n_i32;
        }
        let target_index_usize = target_index as usize;
        result.push(nums[target_index_usize]);
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut tokens = input
        .split_whitespace()
        .map(|token| token.parse::<i32>().expect("Invalid number"));

    let nums_size = tokens.next().expect("No size provided");
    let nums: Vec<i32> = tokens
        .by_ref()
        .take(nums_size as usize)
        .collect();

    // Ensure we have exactly nums_size elements (panics if not, same as original C behavior)
    if nums.len() != nums_size as usize {
        panic!("Insufficient elements provided");
    }

    let transformed = construct_transformed_array(&nums);

    // Print the result with space-separated integers followed by a newline
    for &num in &transformed {
        print!("{} ", num);
    }
    println!();
}