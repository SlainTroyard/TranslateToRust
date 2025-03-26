use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

/// A node in the grid representing a state in our search.
/// `time` is the current time cost to reach the cell at (row, col).
#[derive(Eq, PartialEq, Debug)]
struct Node {
    time: i32,
    row: usize,
    col: usize,
}

/// Implement natural ordering for Node based on time, row and col.
/// Note: We will wrap Node with `std::cmp::Reverse` when putting them into the BinaryHeap
/// so that the smallest time (and then row, col) pops first.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time
            .cmp(&other.time)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Computes the minimum time needed to reach the bottom-right cell of the grid starting
/// from the top-left cell.
/// The grid `move_time` gives the time after which a cell is available to step on.
fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    // visited matrix to ensure each cell is visited only once (as in original code)
    let mut vis = vec![vec![false; cols]; rows];

    // Using a BinaryHeap with Reverse to simulate a min-heap.
    let mut heap = BinaryHeap::new();
    heap.push(std::cmp::Reverse(Node {
        time: 0,
        row: 0,
        col: 0,
    }));
    vis[0][0] = true;

    // Directions: up, right, down, left.
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_time = 0;

    // Process nodes in the min-heap.
    while let Some(std::cmp::Reverse(node)) = heap.pop() {
        current_time = node.time;
        let r = node.row;
        let c = node.col;

        // If we reached the destination, return the time.
        if r == rows - 1 && c == cols - 1 {
            return current_time;
        }

        // Explore the four possible directions.
        for (dr, dc) in directions.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !vis[nr][nc] {
                    // Calculate nextCost as defined in the original code:
                    // nextCost = 2 - ((nrow+ncol) % 2)
                    let next_cost = 2 - ((nr + nc) % 2) as i32;

                    let new_time = if move_time[nr][nc] >= current_time {
                        move_time[nr][nc] + next_cost
                    } else {
                        current_time + next_cost
                    };

                    // Mark cell as visited and add it to the heap.
                    vis[nr][nc] = true;
                    heap.push(std::cmp::Reverse(Node {
                        time: new_time,
                        row: nr,
                        col: nc,
                    }));
                }
            }
        }
    }

    // If destination is not reachable, return the last processed time.
    current_time
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read the first line: rows and cols.
    reader.read_line(&mut input_line)?;
    let mut parts = input_line.split_whitespace();
    let rows: usize = parts
        .next()
        .expect("Expected number of rows")
        .parse()
        .expect("Invalid number for rows");
    let cols: usize = parts
        .next()
        .expect("Expected number of columns")
        .parse()
        .expect("Invalid number for columns");

    // Read the grid values.
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        let row_data: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid grid value"))
            .collect();
        if row_data.len() != cols {
            panic!("Expected {} columns per row", cols);
        }
        move_time.push(row_data);
    }

    // Call the function to compute minimal time and print the result.
    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}