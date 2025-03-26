// Translation of the C++ solution for LeetCode Weekly Contest 431 Problem 4
// to idiomatic Rust, preserving the exact logic and I/O format.

use std::io;
use std::io::BufRead;

// We'll store pairs as (i32, i32) to match the (first, second) usage from the C++ code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair(i32, i32);

// A struct analogous to the C++ class Solution.
struct Solution;

impl Solution {
    fn maximum_weight(&self, intervals: &Vec<Vec<i32>>) -> Vec<i32> {
        let n_original = intervals.len();

        // Build a vector of pairs: we push an initial (-1, -1), then for each interval [l, r, w],
        // push (l, -1) and (r, i) where i is the index of the interval.
        let mut vec_pairs: Vec<Pair> = Vec::new();
        vec_pairs.push(Pair(-1, -1));
        for (i, seg) in intervals.iter().enumerate() {
            vec_pairs.push(Pair(seg[0], -1));
            vec_pairs.push(Pair(seg[1], i as i32));
        }

        // Sort by the first element of the pair (and second if needed for tiebreaks).
        vec_pairs.sort_by(|a, b| a.cmp(b));

        // Now n will correspond to the size of vec_pairs.
        let n = vec_pairs.len();

        // We'll use a 2D DP array f[i][j] where each entry is a [i64; 5].
        // The array layout mirrors the C++ code:
        //   f[i][j][0] represents "negative of total weight" (cost),
        //   f[i][j][1..=4] represent chosen intervals, with INF if not chosen.
        // We will compare them lexicographically.
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18

        // f[i][j] = best array after processing up to i-th pair with j intervals chosen.
        // We'll initialize them all with [INF; 5].
        let mut f = vec![[[INF; 5]; 5]; n];

        // Initialize f[0]: if we haven't chosen any intervals yet, cost is 0, and no intervals chosen.
        // So f[0][0] = [0, INF, INF, INF, INF].
        for j in 1..=4 {
            f[0][j] = [INF; 5];
        }
        f[0][0] = [0, INF, INF, INF, INF];

        // Dynamic programming
        for i in 1..n {
            // Copy previous values by default (like f[i][j] = f[i-1][j] in C++).
            for j in 0..=4 {
                f[i][j] = f[i - 1][j];
            }

            let idx = vec_pairs[i].1; // second field of the pair
            // If this pair corresponds to the right boundary of some interval (idx >= 0),
            // we try to choose that interval.
            if idx >= 0 {
                let idx = idx as usize;
                let l = intervals[idx][0]; // left boundary of the interval

                // Binary search to find the largest pair index strictly less than l
                let mut head = 0;
                let mut tail = i - 1;
                while head < tail {
                    let mid = (head + tail + 1) >> 1;
                    if vec_pairs[mid].0 < l {
                        head = mid;
                    } else {
                        tail = mid - 1;
                    }
                }

                // Try to transition from f[head][j-1] to f[i][j] by choosing this interval
                for j in 1..=4 {
                    let mut tmp = f[head][j - 1];
                    // We subtract the weight from tmp[0] because the C++ code stores negative weight there.
                    tmp[0] -= intervals[idx][2] as i64; // intervals[idx][2] is the weight
                    // Place the chosen interval's index in tmp[j], then sort to maintain lexicographical order
                    tmp[j] = idx as i64;
                    tmp.sort();

                    // Compare lexicographically, update if better
                    if tmp < f[i][j] {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        // Now we compute the answer by taking the lexicographically smallest among f[n-1][j] for j = 1..4
        let mut ans = [INF; 5];
        for j in 1..=4 {
            if f[n - 1][j] < ans {
                ans = f[n - 1][j];
            }
        }

        // Gather the chosen intervals from ans[1..=4], skipping those that are INF
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }

        ret
    }
}

fn main() {
    // Read the input in the exact same format as the C++ code:
    // 1) Read an integer n
    // 2) Then read n lines, each with three integers: [l, r, w]
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = {
        let line = lines.next().unwrap().unwrap();
        line.trim().parse().unwrap()
    };

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // Each interval is [l, r, w]
        intervals.push(vec![parts[0], parts[1], parts[2]]);
    }

    // Solve
    let sol = Solution;
    let ans = sol.maximum_weight(&intervals);

    // Print the result exactly as in the C++ code: each index on one line separated by spaces, then a newline.
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}