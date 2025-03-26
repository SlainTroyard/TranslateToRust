use std::io::{self, BufRead};
use std::cmp::{min, max};

fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort();

    for &num in arr.iter() {
        // Calculate the next distinct value that satisfies our constraints
        // We take the maximum of (prev + 1) and (num - diff)
        let x = max(prev + 1, num - diff);

        // If this value is within range of the current element, we can use it
        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read array size
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse array size");
    
    // Read difference value
    let diff: i32 = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse difference value");
    
    // Read array elements
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap()?.trim().parse()
            .expect("Failed to parse array element");
        arr.push(num);
    }
    
    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}