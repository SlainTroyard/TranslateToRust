use std::collections::HashSet;
use std::io;

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Call the function and output the result
    let result = minimum_operations(&nums);
    println!("{}", result);
}

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    for i in (0..nums.len()).rev() {
        if !seen.insert(nums[i]) {
            return (i / 3 + 1) as i32;
        }
    }
    0
}