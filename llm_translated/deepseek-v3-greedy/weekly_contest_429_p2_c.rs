use std::cmp;
use std::io;

fn max_distinct_elements(mut arr: Vec<i32>, diff: i32) -> i32 {
    // Sort the array in ascending order
    arr.sort();

    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    for &num in &arr {
        // Calculate the minimum value that can be assigned to x
        let x = cmp::max(prev + 1, num - diff);

        // Check if x is within the allowed range
        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    let mut input = String::new();

    // Read the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the difference value
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let diff: i32 = input.trim().parse().expect("Invalid input");

    // Read the array elements
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        arr.push(num);
    }

    // Compute the result
    let result = max_distinct_elements(arr, diff);

    // Output the result
    println!("{}", result);
}