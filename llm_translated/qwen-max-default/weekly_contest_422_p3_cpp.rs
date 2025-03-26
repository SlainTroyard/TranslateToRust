use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    time: i32,
    row: usize,
    col: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.time.cmp(&self.time)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut vis = vec![vec![false; cols]; rows];
        let mut pq = BinaryHeap::new();

        pq.push(State { time: 0, row: 0, col: 0 });
        let drow = [-1, 0, 1, 0];
        let dcol = [0, 1, 0, -1];
        let mut time = 0;
        vis[0][0] = true;

        while let Some(State { time, row, col }) = pq.pop() {
            if row == rows - 1 && col == cols - 1 {
                return time;
            }
            for i in 0..4 {
                let nrow = (row as isize + drow[i]) as usize;
                let ncol = (col as isize + dcol[i]) as usize;
                if nrow < rows && ncol < cols && !vis[nrow][ncol] {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let next_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };
                    pq.push(State { time: next_time, row: nrow, col: ncol });
                    vis[nrow][ncol] = true;
                }
            }
        }
        time
    }
}

fn main() {
    // Read matrix dimensions
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let rows = parts[0];
    let cols = parts[1];

    // Read matrix elements
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        move_time[i] = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    }

    // Calculate result
    let sol = Solution;
    let result = sol.min_time_to_reach(&move_time);
    println!("{}", result);
}