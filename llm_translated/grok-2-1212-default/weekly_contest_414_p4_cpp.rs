use std::io::{self, BufRead};

// Directions for the knight's movement (8 possible directions)
const DIRS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];

struct Solution;

impl Solution {
    // Function to calculate the maximum number of moves
    fn max_moves(&self, kx: i32, ky: i32, positions: &Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let (px, py) = (positions[i][0] as usize, positions[i][1] as usize);
            dis[i][px][py] = 0;
            let mut q = vec![(px, py)];
            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            for step in 1.. {
                let mut tmp = Vec::new();
                for &(qx, qy) in &q {
                    for &(dx, dy) in &DIRS {
                        let x = (qx as i32 + dx) as usize;
                        let y = (qy as i32 + dy) as usize;
                        // Ensure the new position is within bounds and not yet visited
                        if x < 50 && y < 50 && dis[i][x][y] < 0 {
                            dis[i][x][y] = step;
                            tmp.push((x, y));
                        }
                    }
                }
                if tmp.is_empty() {
                    break;
                }
                q = tmp;
            }
        }

        // Add the knight's position to the list of positions
        let mut positions = positions.clone();
        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![-1; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
        let u = (1 << n) - 1;

        // Use recursion to solve the problem
        fn dfs(i: usize, mask: usize, positions: &Vec<Vec<i32>>, dis: &Vec<Vec<Vec<i32>>>, memo: &mut Vec<Vec<i32>>) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            let res = &mut memo[i][mask]; // Use reference to store the result
            if *res != -1 { // If the result has been computed before, return it
                return *res;
            }
            let (x, y) = (positions[i][0] as usize, positions[i][1] as usize);
            if (u ^ mask).count_ones() % 2 == 0 { // Alice's move
                *res = 0;
                for j in 0..n {
                    if mask & (1 << j) != 0 {
                        *res = (*res).max(dfs(j, mask ^ (1 << j), positions, dis, memo) + dis[j][x][y]);
                    }
                }
            } else { // Bob's move
                *res = i32::MAX;
                for j in 0..n {
                    if mask & (1 << j) != 0 {
                        *res = (*res).min(dfs(j, mask ^ (1 << j), positions, dis, memo) + dis[j][x][y]);
                    }
                }
            }
            *res
        }
        dfs(n, u, &positions, &dis, &mut memo) // Start the DFS from the knight's position
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read knight's position (kx, ky) and number of soldiers
    let first_line = lines.next().unwrap()?;
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let kx = nums.next().unwrap();
    let ky = nums.next().unwrap();
    let n = nums.next().unwrap();

    // Read the positions of soldiers
    let mut positions = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        positions.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }

    // Call the max_moves function and output the result
    let solution = Solution;
    let result = solution.max_moves(kx, ky, &positions);
    println!("{}", result);

    Ok(())
}