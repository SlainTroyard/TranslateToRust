use std::collections::HashMap;
use std::io;

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // Call the function and print the result
    let result = get_largest_outlier(&nums);
    println!("{}", result);
}

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let mut ctr = HashMap::new();
    let mut sm = 0;

    // Count occurrences and calculate the sum
    for &num in nums {
        *ctr.entry(num).or_insert(0) += 1;
        sm += num;
    }

    // Collect unique candidates
    let mut cands: Vec<i32> = ctr.keys().cloned().collect();

    // Sort candidates in descending order
    cands.sort_by(|a, b| b.cmp(a));

    // Check each candidate to find the largest outlier
    for &n in &cands {
        let d = (sm - n) / 2;
        let m = (sm - n) % 2;
        if m == 0 && ctr.contains_key(&d) && (d != n || ctr[&d] > 1) {
            return n;
        }
    }

    // If no outlier found, return -1
    -1
}