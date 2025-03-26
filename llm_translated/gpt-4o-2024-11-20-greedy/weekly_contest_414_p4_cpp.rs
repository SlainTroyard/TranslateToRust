use std::collections::VecDeque;

struct Solution;

impl Solution {
    // Directions for the knight's movement (8 possible directions)
    const DIRS: [(i32, i32); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    // Function to calculate the maximum number of moves
    fn max_moves(kx: i32, ky: i32, mut positions: Vec<(i32, i32)>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let (px, py) = positions[i];
            dis[i][px as usize][py as usize] = 0;
            let mut q = VecDeque::new();
            q.push_back((px, py));

            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            let mut step = 1;
            while !q.is_empty() {
                let mut tmp = VecDeque::new();
                while let Some((qx, qy)) = q.pop_front() {
                    for &(dx, dy) in Self::DIRS.iter() {
                        let x = qx + dx;
                        let y = qy + dy;
                        if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] < 0 {
                            dis[i][x as usize][y as usize] = step;
                            tmp.push_back((x, y));
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push((kx, ky));
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;

        // Recursive function to calculate the maximum moves
        fn dfs(
            i: usize,
            mask: usize,
            positions: &Vec<(i32, i32)>,
            dis: &Vec<Vec<Vec<i32>>>,
            n: usize,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            if memo[i][mask] != -1 {
                return memo[i][mask];
            }

            let (x, y) = positions[i];
            if ((mask.count_ones() & 1) == 0) { // Alice's move
                let mut res = 0;
                for j in 0..n {
                    if (mask >> j) & 1 != 0 {
                        res = res.max(dfs(j, mask ^ (1 << j), positions, dis, n, memo) + dis[j][x as usize][y as usize]);
                    }
                }
                memo[i][mask] = res;
            } else { // Bob's move
                let mut res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 != 0 {
                        res = res.min(dfs(j, mask ^ (1 << j), positions, dis, n, memo) + dis[j][x as usize][y as usize]);
                    }
                }
                memo[i][mask] = res;
            }
            memo[i][mask]
        }

        dfs(n, u, &positions, &dis, n, &mut memo)
    }
}

fn main() {
    use std::io::{self, BufRead};

    // I/O handling
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_split = first_line.split_whitespace();
    let kx: i32 = first_split.next().unwrap().parse().unwrap();
    let ky: i32 = first_split.next().unwrap().parse().unwrap();
    let n: usize = first_split.next().unwrap().parse().unwrap();

    // Read positions
    let mut positions = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        let px: i32 = split.next().unwrap().parse().unwrap();
        let py: i32 = split.next().unwrap().parse().unwrap();
        positions.push((px, py));
    }

    // Create Solution instance and compute result
    let solution = Solution;
    let result = solution.max_moves(kx, ky, positions);

    // Output the result
    println!("{}", result);
}