use std::io;
use std::collections::BTreeSet;

struct Solution;

impl Solution {
    /// Finds the maximum number of distinct elements in the array such that the difference
    /// between any two elements is at most `diff`.
    fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
        let mut prev = i32::MIN;
        let mut c = BTreeSet::new();
        arr.sort_unstable();

        for &x in arr.iter() {
            let candidate = (prev + 1).max(x - diff);

            if candidate <= x + diff {
                c.insert(candidate);
                prev = candidate;
            }
        }

        c.len()
    }
}

fn main() {
    let mut input = String::new();

    // Read the size of the array and the difference
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let diff: i32 = input.trim().parse().expect("Please type a number!");
    input.clear();

    // Read the array elements
    let mut arr = vec![0; n];
    for i in 0..n {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please type a number!");
        input.clear();
    }

    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}