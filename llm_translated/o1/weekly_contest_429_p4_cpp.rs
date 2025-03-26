use std::io::{self, BufRead};

/// A struct representing our solution, holding no state.
#[derive(Default)]
struct Solution;

impl Solution {
    /// Merges overlapping intervals under the constraint that
    /// the new interval length requirement `(currStart - lastStart + 1) <= len` is satisfied.
    fn merge(&self, intervals: &[[i32; 2]], len: i32) -> Vec<[i32; 2]> {
        if intervals.is_empty() {
            return Vec::new();
        }

        let mut res = vec![intervals[0]];
        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];
            let last = res.last_mut().unwrap();

            // If current interval can be merged with the last interval in res
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                last[1] = last[1].max(curr_end);
            } else {
                res.push(intervals[i]);
            }
        }
        res
    }

    /// Checks if it is possible to ensure that any continuous substring of length `mid`
    /// can have at least one '0' and one '1' by merging intervals (single-type substrings)
    /// into at most `op` merged intervals.
    fn is_poss(&self, s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();

        // Use a sliding window of size up to `mid` over the binary string
        while j < n {
            if s.as_bytes()[j] == b'0' {
                zero += 1;
            } else {
                one += 1;
            }

            // If the window exceeds `mid`, shrink it from the left
            while (j - i + 1) as i32 > mid {
                if s.as_bytes()[i] == b'0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            // Whenever the window is exactly size `mid`,
            // check if we have all '0's or all '1's
            if (j - i + 1) as i32 == mid {
                if zero == 0 || one == 0 {
                    intervals.push([i as i32, j as i32]);
                }
            }

            j += 1;
        }

        // Merge these intervals to see if we can form at most `op` intervals
        let merged = self.merge(&intervals, mid);
        (merged.len() as i32) <= op
    }

    /// Gets the minimum operations required to transform the string `s`
    /// into an alternating pattern of even/odd indices (e.g., "010101..." or "101010...").
    fn get_mini(&self, s: &str, even: u8, odd: u8) -> i32 {
        let n = s.len();
        let mut op = 0;
        for i in 0..n {
            let expected = if i % 2 == 0 { even } else { odd };
            if s.as_bytes()[i] != expected {
                op += 1;
            }
        }
        op
    }

    /// Finds the minimal length `L` such that there exists a way to make every substring
    /// of length `L` contain at least one '0' and one '1' using no more than `num_ops` operations.
    fn min_length(&self, s: &str, num_ops: i32) -> i32 {
        let n = s.len() as i32;
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        // First, check if we can achieve length 1 by converting
        // the entire string into an alternating pattern.
        let mut op = n + 1;
        op = op.min(self.get_mini(s, b'0', b'1')); // cost to make pattern "010101..."
        op = op.min(self.get_mini(s, b'1', b'0')); // cost to make pattern "101010..."
        if op <= num_ops {
            return 1;
        }

        // Use binary search on the length from 3 to n
        while start <= end {
            let mid = start + (end - start) / 2;
            let can_do = self.is_poss(s, num_ops, mid);
            if can_do {
                end = mid - 1;
            } else {
                res = mid;
                start = mid + 1;
            }
        }

        res
    }
}

fn main() {
    // Read the binary string and the number of operations from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: binary string
    let s = lines.next().expect("No input found").expect("Failed to read line");
    // Second line: number of operations
    let num_ops_line = lines.next().expect("Not enough input lines").expect("Failed to read line");
    let num_ops: i32 = num_ops_line.trim().parse().expect("Failed to parse numOps as i32");

    // Create our solution and compute the result
    let solution = Solution::default();
    let result = solution.min_length(&s, num_ops);

    // Output the result
    println!("{}", result);
}