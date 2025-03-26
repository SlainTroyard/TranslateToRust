use std::cmp;
use std::collections::VecDeque;
use std::io::{self, BufRead};

// Constants, as in C code.
const INF: i32 = i32::MAX;
const N: usize = 50; // board size is 50x50

// Structure representing a knight's position.
#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

// Structure to hold the problem's solution state.
// pos: positions of knights, last one being the extra starting position (kx, ky)
// dist: precomputed distances between all positions (from pos[i] to pos[j] for j in [0, n))
// n: number of "target" positions (from input), excluding the final extra position.
struct Solution {
    pos: Vec<Position>,
    dist: Vec<Vec<i32>>, // dimensions: (n + 1) x n.
    n: usize,            // number of positions from input (not including the extra (kx,ky))
}

// Check if (x, y) is a valid board position.
fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

// Calculate distances between positions using knight moves.
// For each pair (i, j) with i in [0, n+1) and j in [0, n) (skipping same indices),
// compute the minimum number of moves from pos[i] to pos[j] using BFS.
fn calculate_distances(sol: &mut Solution) {
    // All possible moves of a knight.
    let directions = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    // For each source position.
    for i in 0..=sol.n {
        // For each target among the "input" positions.
        for j in 0..sol.n {
            if i == j {
                continue;
            }
            // BFS initialization.
            let mut queue = VecDeque::new();
            queue.push_back(sol.pos[i]);
            // seen[x][y]: whether the cell (x,y) has been visited.
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x as usize][sol.pos[i].y as usize] = true;
            let mut steps = 0;
            // Use a label so we can break out of nested loops when we found the target.
            'bfs: loop {
                let level_size = queue.len();
                if level_size == 0 {
                    break;
                }
                for _ in 0..level_size {
                    let current = queue.pop_front().unwrap();
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        // Go to next (i,j) pair.
                        break 'bfs;
                    }
                    for (dx, dy) in directions.iter() {
                        let nx = current.x + dx;
                        let ny = current.y + dy;
                        if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                            queue.push_back(Position { x: nx, y: ny });
                            seen[nx as usize][ny as usize] = true;
                        }
                    }
                }
                steps += 1;
            }
        }
    }
}

// Type alias for the memoization table.
// dp[i][m][turn] is stored as dp[i][m][turn], where:
//   i: current position index (0..(n+1))
//   m: bitmask state (0 .. (1<<n)-1), using positions of the input (not including extra).
//   turn: 0 or 1.
type Memo = Vec<Vec<[Option<i32>; 2]>>;

// DFS with memoization.
// i: current position index.
// m: bitmask representing visited target positions (only targets, not including (kx, ky)).
// turn: current turn (0 or 1). When turn == 0, we try to maximize the result; when turn == 1, minimize.
fn dfs(sol: &Solution, i: usize, m: usize, turn: usize, dp: &mut Memo) -> i32 {
    // If m has all bits set, i.e., all target positions have been visited.
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if let Some(cached) = dp[i][m][turn] {
        return cached;
    }

    // When turn==0 (maximizer), we initialize with 0. When turn==1 (minimizer), we initialize with INF.
    let mut ans = if turn == 0 { 0 } else { INF };
    // Try each target position that has not been visited yet.
    for k in 0..sol.n {
        // Check if the k-th target has been visited.
        if m & (1 << k) == 0 {
            let next_m = m | (1 << k);
            // cost from current position to target k plus subsequent cost.
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, dp);
            if turn == 0 {
                ans = cmp::max(ans, result);
            } else {
                ans = cmp::min(ans, result);
            }
        }
    }
    dp[i][m][turn] = Some(ans);
    ans
}

// Function implementing the main logic, corresponding to maxMoves in the C code.
// kx, ky: the coordinates given in the first line of input.
// positions: list of positions for the targets (each a 2-element array).
fn max_moves(kx: i32, ky: i32, positions: Vec<[i32; 2]>) -> i32 {
    let positions_size = positions.len();
    // Prepare the solution.
    // sol.n is the number of target positions from input.
    let mut sol = Solution {
        n: positions_size,
        pos: Vec::with_capacity(positions_size + 1),
        // dist: dimensions: (positions_size + 1) x positions_size, initially filled with 0.
        dist: vec![vec![0; positions_size]; positions_size + 1],
    };

    // Fill in the target positions.
    for pos in positions {
        sol.pos.push(Position { x: pos[0], y: pos[1] });
    }
    // Last position is (kx, ky).
    sol.pos.push(Position { x: kx, y: ky });

    // Pre-compute distances using BFS.
    calculate_distances(&mut sol);

    // Prepare the memoization table.
    // Dimensions: (sol.pos.len()) x (1 << sol.n) x 2.
    let total_states = 1 << sol.n;
    let mut dp: Memo = vec![vec![[None, None]; total_states]; sol.pos.len()];

    // Starting state: starting from (kx,ky) which is at index sol.n and with no targets visited (mask = 0)
    dfs(&sol, sol.n, 0, 0, &mut dp)
}

fn main() {
    // Use a buffered reader for efficient input handling.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    // Read first line: kx and ky.
    buffer.clear();
    if reader.read_line(&mut buffer).is_err() {
        eprintln!("Error reading input for kx and ky");
        return;
    }
    // Split the line into tokens.
    let tokens: Vec<&str> = buffer.split_whitespace().collect();
    if tokens.len() < 2 {
        eprintln!("Expected two integers for kx and ky");
        return;
    }
    let kx: i32 = tokens[0].parse().unwrap_or_else(|_| {
        eprintln!("Invalid integer for kx");
        std::process::exit(1);
    });
    let ky: i32 = tokens[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid integer for ky");
        std::process::exit(1);
    });

    // Read second line: positionsSize.
    buffer.clear();
    if reader.read_line(&mut buffer).is_err() {
        eprintln!("Error reading input for positionsSize");
        return;
    }
    let positions_size: usize = match buffer.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid integer for positionsSize");
            return;
        }
    };

    // Read each of the subsequent positions.
    let mut positions: Vec<[i32; 2]> = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        buffer.clear();
        if reader.read_line(&mut buffer).is_err() {
            eprintln!("Error reading a position");
            return;
        }
        let tokens: Vec<&str> = buffer.split_whitespace().collect();
        if tokens.len() < 2 {
            eprintln!("Expected two integers for a position");
            return;
        }
        let x: i32 = tokens[0].parse().unwrap_or_else(|_| {
            eprintln!("Invalid integer for position x");
            std::process::exit(1);
        });
        let y: i32 = tokens[1].parse().unwrap_or_else(|_| {
            eprintln!("Invalid integer for position y");
            std::process::exit(1);
        });
        positions.push([x, y]);
    }

    // Compute result.
    let result = max_moves(kx, ky, positions);
    // Print the result exactly as in the C code.
    println!("{}", result);
}