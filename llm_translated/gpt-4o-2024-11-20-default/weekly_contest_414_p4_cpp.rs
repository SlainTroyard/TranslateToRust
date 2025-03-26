use std::collections::VecDeque;
use std::io::{self, BufRead};

// Constants for the knight's movement directions (8 possible directions)
const DIRS: [(i32, i32); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

struct Solution;

impl Solution {
    // Function to calculate the maximum number of moves
    fn max_moves(kx: usize, ky: usize, positions: Vec<(usize, usize)>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for (i, &(px, py)) in positions.iter().enumerate() {
            dis[i][px][py] = 0;
            let mut queue = VecDeque::new();
            queue.push_back((px as i32, py as i32));

            let mut step = 1;
            while !queue.is_empty() {
                let mut next_queue = VecDeque::new();
                for &(qx, qy) in &queue {
                    for &(dx, dy) in &DIRS {
                        let x = qx + dx;
                        let y = qy + dy;
                        if x >= 0 && x < 50 && y >= 0 && y < 50 {
                            let x = x as usize;
                            let y = y as usize;
                            if dis[i][x][y] == -1 {
                                dis[i][x][y] = step;
                                next_queue.push_back((x as i32, y as i32));
                            }
                        }
                    }
                }
                queue = next_queue;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        let mut positions = positions;
        positions.push((kx, ky));
        let mut memo = vec![vec![-1; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
        let u = (1 << n) - 1;

        // DFS function using a closure for recursion
        let dfs = |memo: &mut Vec<Vec<i32>>,
                   dis: &Vec<Vec<Vec<i32>>>,
                   positions: &Vec<(usize, usize)>,
                   n: usize| {
            let mut dfs_internal = std::collections::HashMap::new();
            let dfs_algorithm  = dfs_along!remember the of folded ***!