use std::collections::HashSet;
use std::io;

fn main() {
    // Read the size of the array and the difference
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let diff: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Read the array elements
    let mut arr = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num in input.split_whitespace() {
        arr.push(num.parse().expect("Invalid input"));
    }

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}

fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
    let mut prev = i32::MIN;
    let mut c = HashSet::new();

    // Sort the array
    arr.sort();

    for &num in arr.iter() {
        let x = std::cmp::max(prev + 1, num - diff);

        if x <= num + diff {
            c.insert(x);
            prev = x;
        }
    }

    c.len()
}