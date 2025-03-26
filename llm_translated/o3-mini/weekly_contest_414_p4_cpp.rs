use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io::{self, BufRead};

// Constant board size (50×50 grid)
const BOARD_SIZE: usize = 50;

// The knight's directions (8 possible moves)
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

fn main() -> io::Result<()> {
    // Set up input reading from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing knight's position (kx, ky) and number of soldiers (n).
    // The input format is exactly as the original code.
    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing input"))??;
    // Split by whitespace
    let mut first_iter = first_line.split_whitespace();
    let kx: usize = first_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing kx"))?
        .parse()
        .expect("Invalid integer for kx");
    let ky: usize = first_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing ky"))?
        .parse()
        .expect("Invalid integer for ky");
    let n: usize = first_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing n"))?
        .parse()
        .expect("Invalid integer for n");

    // Read n lines for soldier positions
    let mut positions: Vec<[usize; 2]> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing soldier position"))??;
        let mut iter = line.split_whitespace();
        let x: usize = iter
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing soldier x"))?
            .parse()
            .expect("Invalid integer for soldier x");
        let y: usize = iter
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing soldier y"))?
            .parse()
            .expect("Invalid integer for soldier y");
        positions.push([x, y]);
    }

    // For each soldier, compute the BFS distances on the board from the soldier's starting position.
    // dis[i][x][y] will hold the minimum number of knight moves from soldier i's starting position to (x,y).
    let mut dis: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; BOARD_SIZE]; BOARD_SIZE]; n];
    for (i, pos) in positions.iter().enumerate() {
        let (px, py) = (pos[0], pos[1]);
        // Starting position has distance 0.
        dis[i][px][py] = 0;
        let mut queue = VecDeque::new();
        queue.push_back((px, py));
        // Standard BFS on the 50x50 board.
        while let Some((qx, qy)) = queue.pop_front() {
            let current_step = dis[i][qx][qy];
            for (dx, dy) in DIRS.iter() {
                // compute new positions as signed, then check bounds.
                let nx_signed = qx as i32 + dx;
                let ny_signed = qy as i32 + dy;
                if nx_signed >= 0 && nx_signed < BOARD_SIZE as i32 && ny_signed >= 0 && ny_signed < BOARD_SIZE as i32 {
                    let nx = nx_signed as usize;
                    let ny = ny_signed as usize;
                    // If not visited then queue the new position.
                    if dis[i][nx][ny] == -1 {
                        dis[i][nx][ny] = current_step + 1;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    // Append the knight's position to the list of positions.
    // The knight's starting position is at index n.
    positions.push([kx, ky]);

    // Prepare memoization for DFS.
    // memo[i][mask] = computed result for state (i, mask). We use Option<i32>, where None means not computed.
    let full_mask = if n > 0 { (1 << n) - 1 } else { 0 };
    let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; 1 << n]; n + 1];

    // Start the DFS from the knight's position with soldiers mask = full_mask.
    let result = dfs(n, full_mask, &positions, &dis, &mut memo, full_mask, n);
    println!("{}", result);
    Ok(())
}

/// Recursive DFS function for computing the maximum number of moves.
/// i: Current position index in positions (n is reserved for the knight's position)
/// mask: Bitmask representing soldiers that are still available (each bit j corresponds to soldier j)
/// positions: List of all positions: first n are soldiers, last one is the knight's start.
/// dis: Precomputed BFS distances for each soldier (only indices 0..n are valid)
/// memo: Memoization table, indexed by (i, mask)
/// full_mask: mask for all soldiers (same as (1 << n) - 1)
/// n: number of soldiers
fn dfs(
    i: usize,
    mask: usize,
    positions: &Vec<[usize; 2]>,
    dis: &Vec<Vec<Vec<i32>>>,
    memo: &mut Vec<Vec<Option<i32>>>,
    full_mask: usize,
    n: usize,
) -> i32 {
    // Base case: no remaining soldiers.
    if mask == 0 {
        return 0;
    }
    // Return memoized value if already computed.
    if let Some(val) = memo[i][mask] {
        return val;
    }
    let (x, y) = (positions[i][0], positions[i][1]);
    
    // Determine whose turn it is based on the parity of the number of moves so far.
    // __builtin_parity(u ^ mask) == 0 -> even parity -> Alice's move (maximizing)
    // In Rust, we compute the parity by counting set bits.
    let parity_even = ((full_mask ^ mask).count_ones() & 1) == 0;
    // Initialize result to an extreme value depending on which branch (maximizing or minimizing).
    let mut res = if parity_even { i32::MIN } else { i32::MAX };

    // Try all available soldiers j (represented by bits in mask).
    for j in 0..n {
        if (mask >> j) & 1 == 1 {
            // Calculate the cost to move from soldier j's starting position to current position (x, y)
            let step_cost = dis[j][x][y];
            // Since BFS always sets a valid non-negative step count once reachable, we add DFS from new state.
            let candidate = dfs(j, mask ^ (1 << j), positions, dis, memo, full_mask, n) + step_cost;
            if parity_even {
                res = max(res, candidate);
            } else {
                res = min(res, candidate);
            }
        }
    }
    memo[i][mask] = Some(res);
    res
}