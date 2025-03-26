// Problem: Weekly Contest 429 Problem 2

use std::io;

fn main() {
    // Read the size of the array and the difference value from stdin
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    let mut diff = String::new();
    io::stdin().read_line(&mut diff).expect("Failed to read line");
    let diff: i32 = diff.trim().parse().expect("Please type a number!");

    // Allocate memory for the array
    let mut arr = vec![0; n];

    // Read the array elements from stdin
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please type a number!");
    }

    // Compute the result
    let result = max_distinct_elements(&arr, diff);

    // Output the result
    println!("{}", result);
}

/// Comparator function for sorting in ascending order
fn cmp_int_asc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

/// Function to find the maximum number of distinct elements with the given difference
fn max_distinct_elements(arr: &[i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(cmp_int_asc);

    for &x in &sorted_arr {
        let x = if prev + 1 > x - diff { prev + 1 } else { x - diff };

        if x <= x + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}