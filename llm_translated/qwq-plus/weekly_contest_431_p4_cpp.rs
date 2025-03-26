use std::io::{self, BufRead};

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().trim();
        let parts: Vec<_> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        intervals.push((parts[0], parts[1], parts[2]));
    }

    // Build events
    let mut events = vec![( -1, -1 )];
    for (i, (l, r, _)) in intervals.iter().enumerate() {
        events.push(( *l, -1 ));
        events.push(( *r, i as i32 ));
    }
    events.sort_unstable();

    let n_events = events.len();
    let mut f = vec![[[INF;5];5]; n_events];

    // Initialize f[0]
    for j in 0..5 {
        let mut arr = [INF;5];
        if j == 0 {
            arr[0] = 0;
        }
        f[0][j] = arr;
    }

    // Dynamic programming
    for i in 1..n_events {
        // Copy previous state
        for j in 0..5 {
            f[i][j] = f[i-1][j];
        }

        let (time, idx) = events[i];
        if idx >= 0 {
            let l = intervals[idx as usize].0;
            let mut head = 0;
            let mut tail = i -1;
            while head < tail {
                let mid = (head + tail + 1) / 2;
                if events[mid].0 < l {
                    head = mid;
                } else {
                    tail = mid -1;
                }
            }

            for j in 1..=4 {
                let prev_j = j - 1;
                let prev = f[head][prev_j];
                if prev[0] == INF {
                    continue;
                }
                let mut tmp = prev;
                tmp[0] -= intervals[idx as usize].2 as i64;
                tmp[j] = idx as i64;
                tmp.sort();
                if tmp < f[i][j] {
                    f[i][j] = tmp;
                }
            }
        }
    }

    // Find the best solution
    let mut ans = [INF;5];
    for j in 1..=4 {
        if f[n_events-1][j] < ans {
            ans = f[n_events-1][j];
        }
    }

    // Collect the indices
    let mut ret = Vec::new();
    for &x in &ans[1..] {
        if x < INF {
            ret.push(x as i32);
        }
    }

    // Output the result
    for num in &ret {
        print!("{} ", num);
    }
    println!();
}