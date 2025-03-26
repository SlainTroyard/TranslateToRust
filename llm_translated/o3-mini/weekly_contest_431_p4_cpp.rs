use std::cmp::Ordering;
use std::io::{self, BufRead, Write};
use std::error::Error;

/// INF constant: 1e18
const INF: i64 = 1_000_000_000_000_000_000 / 1000; // 1e18

/// A structure to hold our solution.
struct Solution;

impl Solution {
    /// Computes the maximum weight selection from intervals.
    ///
    /// # Arguments
    /// * `intervals` - A vector of intervals with each interval represented as [l, r, cost]
    ///
    /// # Returns
    /// A vector of selected interval indices (as i32) following the solution criteria.
    fn maximum_weight(intervals: &Vec<Vec<i32>>) -> Vec<i32> {
        let n_intervals = intervals.len();
        
        // Create a vector of tuples, each is (coordinate, index)
        // Start with (-1, -1)
        let mut coords: Vec<(i64, i64)> = Vec::new();
        coords.push((-1, -1));
        // For each interval, push (l, -1) and (r, i)
        for (i, seg) in intervals.iter().enumerate() {
            let l = seg[0] as i64;
            let r = seg[1] as i64;
            coords.push((l, -1));
            coords.push((r, i as i64));
        }
        // Sort lexicographically by coordinate and then by index
        coords.sort_by(|a, b| {
            let ord = a.0.cmp(&b.0);
            if ord == Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                ord
            }
        });

        // Let dp state dimension be the size of coords
        let n_dp = coords.len();

        // We'll define dp as a Vec of arrays.
        // Each dp[i] is an array of 5 states (for j = 0..4)
        // and each state is represented as an array of 5 i64 values.
        type State = [i64; 5];
        let mut dp: Vec<[State; 5]> = vec![[[INF; 5]; 5]; n_dp];

        // Initialization: For i = 0:
        // For j from 1 to 4: dp[0][j] = [INF, INF, INF, INF, INF]
        // dp[0][0] = [0, INF, INF, INF, INF]
        dp[0][0] = [0, INF, INF, INF, INF];
        for j in 1..5 {
            dp[0][j] = [INF; 5];
        }

        // Dynamic Programming: Iterate over each coordinate state.
        for i in 1..n_dp {
            // Set dp[i] initially equal to dp[i-1] for all j
            for j in 0..5 {
                dp[i][j] = dp[i - 1][j];
            }

            // If the current coordinate corresponds to an interval's r endpoint
            let idx = coords[i].1;
            if idx >= 0 {
                // The corresponding interval index (from intervals vector)
                let int_idx = idx as usize;
                let l = intervals[int_idx][0] as i64;
                // Binary search on coords[0..i] to find the largest index "head"
                // where coords[head].0 < l.
                let mut head = 0;
                let mut tail = i - 1;
                while head < tail {
                    let mid = (head + tail + 1) / 2;
                    if coords[mid].0 < l {
                        head = mid;
                    } else {
                        // coords[mid].0 >= l, so we search left side
                        tail = mid - 1;
                    }
                }

                // For every j from 1 to 4, try updating dp[i][j]
                for j in 1..5 {
                    // Start with a copy of dp[head][j-1]
                    let mut tmp = dp[head][j - 1];
                    // Subtract the cost from the first element (weight adjustment)
                    tmp[0] -= intervals[int_idx][2] as i64;
                    // Mark the current interval index in the j-th position.
                    tmp[j] = idx; // idx is stored as i64

                    // Sort tmp array in non-decreasing order (lexicographical order)
                    // Convert to mutable slice and sort.
                    tmp.sort();

                    // dp[i][j] = minimum of current value and tmp (using lexicographical comparison)
                    if tmp < dp[i][j] {
                        dp[i][j] = tmp;
                    }
                }
            }
        }

        // Retrieve answer from dp[n_dp - 1]
        // We want the lexicographical minimum over j from 1 to 4.
        let mut ans: State = [INF; 5];
        for j in 1..5 {
            if dp[n_dp - 1][j] < ans {
                ans = dp[n_dp - 1][j];
            }
        }

        // Collect IDs from ans[1..4] that are valid (i.e. less than INF)
        let mut ret: Vec<i32> = Vec::new();
        for j in 1..5 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        ret
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up standard input and output
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    // Read the entire input into a String (this code matches the C++ style input)
    let mut input = String::new();
    stdin_lock.read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // First token is the number of intervals
    let n: usize = tokens
        .next()
        .ok_or("Expected number of intervals")?
        .parse()?;
    
    // Read each interval (3 integers per interval)
    let mut intervals: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let l: i32 = tokens.next().ok_or("Expected l")?.parse()?;
        let r: i32 = tokens.next().ok_or("Expected r")?.parse()?;
        let cost: i32 = tokens.next().ok_or("Expected cost")?.parse()?;
        intervals.push(vec![l, r, cost]);
    }

    // Solve the problem using the maximum_weight method
    let ans = Solution::maximum_weight(&intervals);

    // Output the answer with a space between numbers, matching the original C++ output
    for num in &ans {
        write!(out, "{} ", num)?;
    }
    writeln!(out)?;

    Ok(())
}