use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

const N: usize = 50;
const INF: i32 = i32::MAX;

/// Represents a position on the chessboard.
#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

/// Holds all necessary information for our solution:
/// - n is the number of special positions to visit
/// - pos is a vector of positions (n + 1 because the last one is the knight's starting position)
/// - dist is a precomputed distance table:
///   dist[i][j] = BFS distance from pos[i] to pos[j] (0 <= i <= n, 0 <= j < n)
struct Solution {
    n: usize,
    pos: Vec<Position>,
    dist: [[i32; N]; N + 1], // dimension: (N+1) x N, matching the C code
}

/// Calculate all pairwise distances from each position (including the knight's start)
/// to all other "positions" via BFS on a 50x50 board using knight moves.
fn calculate_distances(sol: &mut Solution) {
    // All possible knight moves
    let directions = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1),
    ];

    // Check if a position is valid on the 50x50 board
    let is_valid = |x: i32, y: i32| -> bool {
        x >= 0 && x < N as i32 && y >= 0 && y < N as i32
    };

    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j {
                // Skip distance from a position to itself
                continue;
            }
            // BFS from pos[i] to pos[j]
            let start = sol.pos[i];
            let goal = sol.pos[j];

            let mut queue = VecDeque::new();
            queue.push_back(start);

            let mut seen = [[false; N]; N]; // 50x50 board
            seen[start.x as usize][start.y as usize] = true;

            let mut steps = 0;
            let mut found = false;

            while !queue.is_empty() && !found {
                let layer_size = queue.len();
                for _ in 0..layer_size {
                    let current = queue.pop_front().unwrap();
                    if current.x == goal.x && current.y == goal.y {
                        sol.dist[i][j] = steps;
                        found = true;
                        break;
                    }
                    // Try all knight moves
                    for &(dx, dy) in &directions {
                        let nx = current.x + dx;
                        let ny = current.y + dy;
                        if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                            seen[nx as usize][ny as usize] = true;
                            queue.push_back(Position { x: nx, y: ny });
                        }
                    }
                }
                steps += 1;
            }
        }
    }
}

/// DFS with memoization to find either the maximum or minimum distance sum,
/// depending on whose turn it is (turn 0 = maximizing, turn 1 = minimizing).
fn dfs(
    sol: &Solution,
    i: usize,
    m: u64,
    turn: usize,
    memo: &mut Vec<Vec<[i32; 2]>>
) -> i32 {
    // If we have visited all special positions, no more moves can be added.
    if m == (1_u64 << sol.n) - 1 {
        return 0;
    }

    // Return memoized result (if any).
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }

    let mut ans = if turn == 0 { 0 } else { INF };

    // Try moving to any unvisited position k.
    for k in 0..sol.n {
        if (m & (1_u64 << k)) == 0 {
            let next_m = m | (1_u64 << k);
            let cost = sol.dist[i][k];
            let result = cost + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                // Maximizing turn
                if result > ans {
                    ans = result;
                }
            } else {
                // Minimizing turn
                if result < ans {
                    ans = result;
                }
            }
        }
    }

    memo[i][m as usize][turn] = ans;
    ans
}

/// Main logic function, replicating the C code's maxMoves().
/// Positions is expected to be an array of [x, y] pairs for the special positions.
/// kx, ky is the knight's starting position.
fn max_moves(kx: i32, ky: i32, positions: &[[i32; 2]]) -> i32 {
    let mut sol = Solution {
        n: positions.len(),
        pos: Vec::new(),
        dist: [[0; N]; N + 1],
    };

    // Fill sol.pos from the input
    for &p in positions {
        sol.pos.push(Position { x: p[0], y: p[1] });
    }
    // Add the knight's own starting position last
    sol.pos.push(Position { x: kx, y: ky });

    // Precompute BFS distances from each relevant position
    calculate_distances(&mut sol);

    // Prepare memo: dimension (N+1) x (1 << n) x 2
    // memo[i][mask][turn]
    let mut memo = vec![vec![[-1; 2]; 1 << sol.n]; N + 1];

    // Run DFS from the knight's position (sol.n is the index of the knight's start),
    // with mask = 0, turn = 0 (maximizing)
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    // We will parse the input exactly as in the C code:
    // 1) kx ky
    // 2) positionsSize
    // 3) positionsSize lines of two integers each
    // Finally, we print a single integer (the result) followed by a newline.

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read knight's starting position kx, ky
    let first_line = lines.next().unwrap()?;
    let mut tokens = first_line.split_whitespace();
    let kx = tokens.next().unwrap().parse::<i32>().unwrap();
    let ky = tokens.next().unwrap().parse::<i32>().unwrap();

    // Read positionsSize
    let second_line = lines.next().unwrap()?;
    let positions_size = second_line.trim().parse::<usize>().unwrap();

    // Read the positions
    let mut positions = Vec::new();
    for _ in 0..positions_size {
        if let Some(Ok(line)) = lines.next() {
            let mut coords = line.split_whitespace();
            let x = coords.next().unwrap().parse::<i32>().unwrap();
            let y = coords.next().unwrap().parse::<i32>().unwrap();
            positions.push([x, y]);
        }
    }

    // Compute the result
    let result = max_moves(kx, ky, &positions);

    // Print the result (same format as C code)
    println!("{}", result);

    Ok(())
}