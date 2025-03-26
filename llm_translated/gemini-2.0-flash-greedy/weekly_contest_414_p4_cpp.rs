use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

const DIRS: [[i32; 2]; 8] = [
    [2, 1],
    [1, 2],
    [-1, 2],
    [-2, 1],
    [-2, -1],
    [-1, -2],
    [1, -2],
    [2, -1],
];

struct Solution {}

impl Solution {
    fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        let mut dis = [[[(-1) as i32; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let px = positions[i][0] as usize;
            let py = positions[i][1] as usize;
            dis[i][px][py] = 0;

            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            q.push_back((px, py));

            let mut step = 1;
            while !q.is_empty() {
                let mut tmp: VecDeque<(usize, usize)> = VecDeque::new();

                for _ in 0..q.len() {
                    if let Some((qx, qy)) = q.pop_front() {
                        for &[dx, dy] in &DIRS {
                            let x = qx as i32 + dx;
                            let y = qy as i32 + dy;

                            // Ensure the new position is within bounds and not yet visited
                            if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] < 0
                            {
                                dis[i][x as usize][y as usize] = step;
                                tmp.push_back((x as usize, y as usize));
                            }
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![(-1) as i32; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
        let u = (1 << n) - 1;

        // Use std::function to allow recursion within the lambda
        fn dfs(
            i: usize,
            mask: i32,
            positions: &Vec<Vec<i32>>,
            dis: &[[[i32; 50]; 50]; usize],
            memo: &mut Vec<Vec<i32>>,
            n: usize,
            u: i32,
        ) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            if memo[i][mask as usize] != -1 {
                // If the result has been calculated before, return it
                return memo[i][mask as usize];
            }

            let x = positions[i][0] as usize;
            let y = positions[i][1] as usize;
            let mut res = if (u ^ mask).count_ones() % 2 == 0 {
                // Alice's move
                i32::MIN
            } else {
                // Bob's move
                i32::MAX
            };

            for j in 0..n {
                if (mask >> j) & 1 != 0 {
                    let val = dfs(
                        j,
                        mask ^ (1 << j),
                        positions,
                        dis,
                        memo,
                        n,
                        u,
                    ) + dis[j][x][y];

                    if (u ^ mask).count_ones() % 2 == 0 {
                        // Alice's move
                        res = max(res, val);
                    } else {
                        // Bob's move
                        res = min(res, val);
                    }
                }
            }

            memo[i][mask as usize] = res;
            res
        }
        dfs(
            n,
            u,
            &positions,
            &dis,
            &mut memo,
            n,
            u,
        ) // Start the DFS from the knight's position
    }
}

fn main() -> io::Result<()> {
    let mut kx = String::new();
    io::stdin().read_line(&mut kx).expect("Failed to read line");
    let mut kx = kx.trim().split_whitespace();
    let kx_val: i32 = kx.next().unwrap().parse().unwrap();
    let ky_val: i32 = kx.next().unwrap().parse().unwrap();

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().unwrap();

    let mut positions: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut nums = line.trim().split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }

    let solution = Solution {};
    let result = solution.max_moves(kx_val, ky_val, positions);
    println!("{}", result);

    Ok(())
}