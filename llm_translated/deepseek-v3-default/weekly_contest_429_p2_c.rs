use std::cmp;
use std::io;

// Comparator function for sorting integers in ascending order
fn cmp_int_asc(a: &i32, b: &i32) -> cmp::Ordering {
    a.cmp(b)
}

// Function to calculate the maximum number of distinct elements
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort_by(cmp_int_asc);

    for &num in arr.iter() {
        let x = cmp::max(prev + 1, num - diff);

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

    // Allocate memory for the array
    let mut arr = Vec::with_capacity(n);

    // Read the array elements
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        arr.push(num);
    }

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}