use std::collections::HashSet;

fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    for (rev_index, &num) in nums.iter().rev().enumerate() {
        if !seen.insert(num) {
            // Calculate original index from reversed iteration position
            let original_i = (nums.len() - 1) - rev_index;
            return (original_i / 3 + 1) as i32;
        }
    }
    0
}

fn main() {
    // Read entire input and split into whitespace-separated tokens
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse array size and elements following original C++ input format
    let n: usize = tokens
        .next()
        .expect("Missing array size")
        .parse()
        .expect("Invalid array size");
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid array element"))
        .collect();

    println!("{}", minimum_operations(nums));
}