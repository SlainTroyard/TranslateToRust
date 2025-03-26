use std::io;

/// A function to read an integer from stdin.
fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

/// A function to read an array of integers from stdin.
fn read_int_array(size: usize) -> Vec<i32> {
    let mut arr = Vec::with_capacity(size);
    for _ in 0..size {
        arr.push(read_int());
    }
    arr
}

/// The main logic to find the maximum number of distinct elements with a given difference.
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    arr.sort_unstable(); // Sort the array in ascending order

    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    for &x in arr.iter() {
        let x_val = if prev + 1 > x - diff { prev + 1 } else { x - diff };

        if x_val <= x + diff {
            distinct_count += 1;
            prev = x_val;
        }
    }

    distinct_count
}

fn main() {
    // Read the size of the array and the difference
    let n = read_int();
    let diff = read_int();

    // Read the array elements
    let mut arr = read_int_array(n as usize);

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}