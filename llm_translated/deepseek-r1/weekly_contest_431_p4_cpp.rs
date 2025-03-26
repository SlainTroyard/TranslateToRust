use std::cmp::Ordering;
use std::io::{self, BufRead};

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        intervals.push([parts[0], parts[1], parts[2]]);
    }

    let result = maximum_weight(intervals);

    for num in result {
        print!("{} ", num);
    }
    println!();
}

fn maximum_weight(intervals: Vec<[i32; 3]>) -> Vec<i32> {
    let mut events = vec![(-1i32, -1i32)];
    for (i, &[s, e, _]) in intervals.iter().enumerate() {
        events.push((s, -1));            // Start event marked with -1
        events.push((e, i as i32));      // End event with interval index
    }
    events.sort();

    let n = events.len();
    let mut f = vec![[[INF; 5]; 5]; n];  // DP table: [position][selected_count][state]

    // Initialize base case: 0 intervals selected at position 0
    f[0][0] = [0, INF, INF, INF, INF];
    for j in 1..5 {
        f[0][j] = [INF; 5];
    }

    for i in 1..n {
        // Start with previous state
        f[i] = f[i - 1];

        let (pos, idx) = events[i];
        if idx >= 0 {
            let idx = idx as usize;
            let [l, r, w] = intervals[idx];

            // Binary search for latest event before current interval's start
            let mut head = 0;
            let mut tail = i - 1;
            while head < tail {
                let mid = (head + tail + 1) / 2;
                if events[mid].0 < l {
                    head = mid;
                } else {
                    tail = mid - 1;
                }
            }

            // Update DP states for selecting 1-4 intervals
            for j in 1..=4 {
                let prev_j = j - 1;
                let prev_state = f[head][prev_j];
                let mut new_state = prev_state;
                new_state[0] -= w as i64;  // Accumulate negative weight for min comparison
                new_state[j] = idx as i64; // Store interval index
                new_state.sort();          // Maintain lex order for min comparison

                if new_state < f[i][j] {
                    f[i][j] = new_state;
                }
            }
        }
    }

    // Find the best state across all possible selected counts (1-4)
    let mut best = [INF; 5];
    for j in 1..=4 {
        best = std::cmp::min(best, f[n - 1][j]);
    }

    // Collect valid interval indices (those < INF)
    let mut result = Vec::new();
    for j in 1..=4 {
        if best[j] < INF {
            result.push(best[j] as i32);
        }
    }

    result
}