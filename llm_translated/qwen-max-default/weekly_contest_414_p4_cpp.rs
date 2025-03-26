use std::io::{self, BufRead, Write};

// Directions for the knight's movement (8 possible directions)
const DIRS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];

struct Solution;

impl Solution {
    // Function to calculate the maximum number of moves
    fn max_moves(kx: usize, ky: usize, positions: &mut Vec<(usize, usize)>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let (px, py) = positions[i];
            dis[i][px][py] = 0;
            let mut q = vec![(px, py)];
            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            let mut step = 1;
            while !q.is_empty() {
                let mut tmp = Vec::new();
                for (qx, qy) in q.iter() {
                    for &(dx, dy) in DIRS.iter() {
                        let x = qx as i32 + dx;
                        let y = qy as i32 + dy;
                        if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] < 0 {
                            dis[i][x as usize][y as usize] = step;
                            tmp.push((x as usize, y as usize));
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push((kx, ky));
        let mut memo = vec![vec![-1; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
        let u = (1 << n) - 1;

        // Use a closure to allow recursion within the function
        fn dfs(i: usize, mask: usize, positions: &Vec<(usize, usize)>, dis: &Vec<Vec<Vec<i32>>>, memo: &mut Vec<Vec<i32>>, u: usize) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            let res = &mut memo[i][mask];
            if *res != -1 {
                return *res; // If the result has been calculated before, return it
            }
            let (x, y) = positions[i];
            if (u ^ mask).count_ones() % 2 == 0 { // Alice's move
                for j in 0..positions.len() - 1 {
                    if mask & (1 << j) != 0 {
                        *res = (*res).max(dfs(j, mask ^ (1 << j), positions, dis, memo, u) + dis[j][x][y]);
                    }
                }
            } else { // Bob's move
                *res = i32::MAX;
                for j in 0..positions.len() - 1 {
                    if mask & (1 << j) != 0 {
                        *res = (*res).min(dfs(j, mask ^ (1 << j), positions, dis, memo, u) + dis[j][x][y]);
                    }
                }
            }
            *res
        }

        dfs(n, u, positions, &dis, &mut memo, u) // Start the DFS from the knight's position
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let input: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (kx, ky, n) = (input[0], input[1], input[2]);

    let mut positions = Vec::new();
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let pos: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        positions.push((pos[0], pos[1]));
    }

    let result = Solution::max_moves(kx, ky, &mut positions);
    writeln!(stdout_lock, "{}", result).unwrap();
}