use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: usize,
    col: usize,
}

// Implement `Ord` and `PartialOrd` for `HeapNode` to behave as a min-heap in a `BinaryHeap`.
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time).reverse() // Reverse for min-heap behavior
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Visited matrix
    let mut visited = vec![vec![false; cols]; rows];

    // Min-heap for BFS
    let mut min_heap = BinaryHeap::new();

    // Push the starting point
    min_heap.push(HeapNode { time: 0, row: 0, col: 0 });
    visited[0][0] = true;

    // Direction arrays
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while let Some(current_node) = min_heap.pop() {
        time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        // If destination is reached
        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        // Check all four directions
        for i in 0..4 {
            let nrow = r as isize + drow[i];
            let ncol = c as isize + dcol[i];

            // Check bounds and visited
            if nrow >= 0 && nrow < rows as isize && ncol >= 0 && ncol < cols as isize {
                let nrow = nrow as usize;
                let ncol = ncol as usize;

                if !visited[nrow][ncol] {
                    let next_cost = 2 - ((nrow + ncol) % 2) as i32;
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };

                    min_heap.push(HeapNode {
                        time: new_time,
                        row: nrow,
                        col: ncol,
                    });
                    visited[nrow][ncol] = true;
                }
            }
        }
    }

    time
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let rows = dimensions[0];
    let cols = dimensions[1];

    let mut move_time = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let row_values: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..cols {
            move_time[i][j] = row_values[j];
        }
    }

    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}