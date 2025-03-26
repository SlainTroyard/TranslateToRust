use std::io::{self, BufRead, Write};
use std::cmp::Ordering;

const MAX_SIZE: usize = 50001;

fn gcd_values(nums: &[usize], nums_size: usize, queries: &[i64], queries_size: usize) -> Vec<usize> {
    let mut mx = 0;

    for &num in nums.iter() {
        if num > mx {
            mx = num;
        }
    }

    // Step 1: Count occurrences of each number
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums.iter() {
        cnt_x[num] += 1;
    }

    // Step 2: Calculate gcd contribution
    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += c as i64 * (c as i64 - 1) / 2;
    }

    // Step 3: Accumulate gcd contributions
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Step 4: Answer queries
    let mut ans = vec![0; queries_size];
    for (query_idx, &query) in queries.iter().enumerate() {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans[query_idx] = left;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = stdin.lock().lines();

    // Step 1: Read numsSize and nums
    let nums_size: usize = input.next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Failed to parse numsSize");

    let nums: Vec<usize> = input.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse nums"))
        .collect();

    assert_eq!(nums.len(), nums_size, "numsSize does not match the input length");

    // Step 2: Read queriesSize and queries
    let queries_size: usize = input.next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Failed to parse queriesSize");

    let queries: Vec<i64> = input.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse queries"))
        .collect();

    assert_eq!(queries.len(), queries_size, "queriesSize does not match the input length");

    // Step 3: Compute the result
    let ans = gcd_values(&nums, nums_size, &queries, queries_size);

    // Step 4: Print the result
    writeln!(stdout, "{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
        .expect("Failed to write the output");
}