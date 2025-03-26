use std::collections::BTreeMap;
use std::io::{self, Read};

fn max_subarray_sum(v: &[i32], k: i32) -> i64 {
    let mut m = BTreeMap::new();
    let mut ans = i64::MIN;
    let mut sm = 0i64;

    for (i, &num) in v.iter().enumerate() {
        sm += num as i64;
        let cur_sz = (i + 1) as i32;

        // If current size is a multiple of k, update answer with current sum
        if cur_sz % k == 0 {
            ans = ans.max(sm);
        }

        let y = cur_sz % k;

        // Check if the same remainder exists in the map
        match m.get(&y) {
            Some(&prev_sm) => {
                // Update answer with current sum minus the previous sum for the same remainder
                ans = ans.max(sm - prev_sm);
                // Keep the minimum sum for the current remainder to maximize future differences
                if sm < prev_sm {
                    m.insert(y, sm);
                }
            }
            None => {
                // Insert current sum for this remainder if it's the first occurrence
                m.insert(y, sm);
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse n and k from input
    let n: usize = tokens
        .next()
        .expect("Missing n")
        .parse()
        .expect("Invalid n");
    let k: i32 = tokens
        .next()
        .expect("Missing k")
        .parse()
        .expect("Invalid k");

    // Parse the array elements
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Calculate and print the result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}