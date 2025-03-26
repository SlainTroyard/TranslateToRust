use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Structure to represent a node in the MinHeap
#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapNode {
    time: i32,
    row: usize,
    col: usize,
}

// Implement Ord and PartialOrd for BinaryHeap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time) // Reverse the order for MinHeap behavior
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Function to calculate the minimum time to reach the target
fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut min_heap = BinaryHeap::new();

    // Push the starting point into the heap
    min_heap.push(Reverse(HeapNode { time: 0, row: 0, col: 0 }));
    visited[0][0] = true;

    // Directions for movement (up, right, down, left)
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some(Reverse(current_node)) = min_heap.pop() {
        let time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        // If this is the destination cell
        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] {
                    // Calculate the cost to enter the next cell
                    let next_cost = 2 - (nr + nc) % 2;
                    let new_time = if move_time[nr][nc] >= time {
                        move_time[nr][nc] + next_cost
                    } else {
                        time + next_cost
                    };

                    // Push the new node into the heap
                    min_heap.push(Reverse(HeapNode { time: new_time, row: nr, col: nc }));
                    visited[nr][nc] = true;
                }
            }
        }
    }

    0 // Default fallback (should never reach here if input is valid)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut dims = input.trim().split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];

    // Read the grid values
    for i in 0..rows {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        move_time[i] = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    // Calculate the result
    let result = min_time_to_reach(move_time);
    println!("{}", result);
}