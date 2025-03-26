use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead, Write};

/// This function implements the algorithm from LeetCode Weekly Contest 419 Problem 4.
/// It computes, for each sliding window of size `k` over `nums`, the sum of the products (count * value)
/// for the x selected numbers according to the algorithm.
fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
    // In the original C++ code, L and R are ordered sets of pairs (count, number).
    // We will use BTreeSet in Rust. Their natural order (by count then value) matches the C++ pair ordering.
    let mut L: BTreeSet<(i32, i32)> = BTreeSet::new(); // Will hold x "largest" pairs
    let mut R: BTreeSet<(i32, i32)> = BTreeSet::new(); // Will hold remaining pairs
    let mut sum_l: i64 = 0; // Sum over pairs in L: each (count * number)
    let mut cnt: HashMap<i32, i32> = HashMap::new(); // Count of each number

    // Closure to add the number with its updated count to the appropriate set.
    let mut add = |num: i32, cnt: &HashMap<i32, i32>,
                   L: &mut BTreeSet<(i32, i32)>,
                   R: &mut BTreeSet<(i32, i32)>,
                   sum_l: &mut i64| {
        let count = *cnt.get(&num).unwrap_or(&0);
        if count == 0 {
            return;
        }
        let p = (count, num);
        // If L is non-empty and the new pair is greater than the smallest of L, add it to L.
        if !L.is_empty() && p > *L.iter().next().unwrap() {
            *sum_l += p.0 as i64 * p.1 as i64;
            L.insert(p);
        } else {
            // Otherwise, the pair is inserted to R.
            R.insert(p);
        }
    };

    // Closure to remove the current pair (based on count and num) from the set it is in.
    let mut del = |num: i32, cnt: &HashMap<i32, i32>,
                   L: &mut BTreeSet<(i32, i32)>,
                   R: &mut BTreeSet<(i32, i32)>,
                   sum_l: &mut i64| {
        let count = *cnt.get(&num).unwrap_or(&0);
        if count == 0 {
            return;
        }
        let p = (count, num);
        if L.contains(&p) {
            *sum_l -= p.0 as i64 * p.1 as i64;
            L.remove(&p);
        } else {
            R.remove(&p);
        }
    };

    // Function to move the smallest element from L to R
    let mut l2r = |L: &mut BTreeSet<(i32, i32)>,
                    R: &mut BTreeSet<(i32, i32)>,
                    sum_l: &mut i64| {
        if let Some(&p) = L.iter().next() {
            *sum_l -= p.0 as i64 * p.1 as i64;
            L.remove(&p);
            R.insert(p);
        }
    };

    // Function to move the largest element from R to L
    let mut r2l = |L: &mut BTreeSet<(i32, i32)>,
                    R: &mut BTreeSet<(i32, i32)>,
                    sum_l: &mut i64| {
        if let Some(&p) = R.iter().next_back() {
            *sum_l += p.0 as i64 * p.1 as i64;
            R.remove(&p);
            L.insert(p);
        }
    };

    let n = nums.len();
    // The answer vector will have one element per valid sliding window.
    let mut ans = vec![0i64; n.saturating_sub(k) + 1];

    // Process each index r as the right endpoint of the current sliding window.
    for r in 0..n {
        let in_num = nums[r];

        // Remove the old state of the incoming number
        del(in_num, &cnt, &mut L, &mut R, &mut sum_l);
        // Increase the count
        *cnt.entry(in_num).or_insert(0) += 1;
        // Add the updated element into the proper set
        add(in_num, &cnt, &mut L, &mut R, &mut sum_l);

        // Determine the left endpoint l of the current window.
        if r + 1 < k {
            continue; // Window hasn't reached size k yet.
        }
        let l = r + 1 - k;

        // Rebalance: Ensure L contains exactly x elements (if possible).
        while !R.is_empty() && L.len() < x {
            r2l(&mut L, &mut R, &mut sum_l);
        }
        while L.len() > x {
            l2r(&mut L, &mut R, &mut sum_l);
        }
        // Record the sum for the current window.
        ans[l] = sum_l;

        // Prepare to slide the window by removing the outgoing element.
        let out_num = nums[l];
        del(out_num, &cnt, &mut L, &mut R, &mut sum_l);
        if let Some(count) = cnt.get_mut(&out_num) {
            *count -= 1;
        }
        add(out_num, &cnt, &mut L, &mut R, &mut sum_l);
    }

    ans
}

/// Reads all whitespace-separated tokens from standard input.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        // Panic on I/O errors.
        let line = line.expect("Failed to read line");
        tokens.extend(line.split_whitespace().map(String::from));
    }
    tokens
}

fn main() -> io::Result<()> {
    // Read all tokens from input.
    let tokens = read_tokens();
    let mut iter = tokens.into_iter();

    // Parse k and x (both on the same line or separate, following input order)
    let k: usize = iter.next().expect("Missing k").parse().expect("Invalid k");
    let x: usize = iter.next().expect("Missing x").parse().expect("Invalid x");

    // Parse the number of elements.
    let nums_size: usize = iter.next().expect("Missing numsSize").parse().expect("Invalid numsSize");
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = iter.next().expect("Missing number").parse().expect("Invalid number");
        nums.push(num);
    }

    // Compute the result.
    let res = find_x_sum(nums, k, x);

    // Output the results in the same format as the original C++ code:
    // each number separated by a space, ending with a newline.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for val in res {
        write!(out, "{} ", val)?;
    }
    writeln!(out)?;
    Ok(())
}