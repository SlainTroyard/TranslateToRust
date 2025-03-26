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

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let kx = tokens.next().unwrap();
    let ky = tokens.next().unwrap();
    let n = tokens.next().unwrap() as usize;
    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let x = tokens.next().unwrap() as usize;
        let y = tokens.next().unwrap() as usize;
        positions.push((x, y));
    }
    let knight_x = kx as usize;
    let knight_y = ky as usize;
    positions.push((knight_x, knight_y));

    // Precompute the distance arrays
    let mut dis: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; 50]; 50]; n]; // For each soldier (0..n)
    for i in 0..n {
        let (px, py) = positions[i];
        dis[i][px][py] = 0;
        let mut q = vec![(px, py)];
        let mut step = 1;
        loop {
            let mut next_q = Vec::new();
            for &(qx, qy) in &q {
                for &(dx, dy) in &DIRS {
                    let x = qx as i32 + dx;
                    let y_coord = qy as i32 + dy;
                    if x >= 0 && x < 50 && y_coord >= 0 && y_coord < 50 {
                        let x_usize = x as usize;
                        let y_usize = y_coord as usize;
                        if dis[i][x_usize][y_usize] == -1 {
                            dis[i][x_usize][y_usize] = step;
                            next_q.push((x_usize, y_usize));
                        }
                    }
                }
            }
            if next_q.is_empty() {
                break;
            }
            q = next_q;
            step += 1;
        }
    }

    let u = (1 << n) - 1;
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; 1 << n]; n + 1]; // (n+1) rows, each with 2^n elements
    let result = dfs(n, u, &mut memo, &positions, &dis, n, u);
    println!("{}", result);
}

fn dfs(
    i: usize,
    mask: usize,
    memo: &mut Vec<Vec<i32>>,
    positions: &[(usize, usize)],
    dis: &[Vec<Vec<i32>>],
    n: usize,
    u: usize,
) -> i32 {
    if mask == 0 {
        return 0;
    }
    let current_memo = &mut memo[i][mask];
    if *current_memo != -1 {
        return *current_memo;
    }

    let (x, y) = positions[i];
    let is_alice = (u ^ mask).count_ones() % 2 == 0;
    let mut res = if is_alice { i32::MIN } else { i32::MAX };

    for j in 0..n {
        if (mask & (1 << j)) != 0 {
            let new_mask = mask ^ (1 << j);
            let candidate = dfs(j, new_mask, memo, positions, dis, n, u) + dis[j][x][y];
            if is_alice {
                res = res.max(candidate);
            } else {
                res = res.min(candidate);
            }
        }
    }

    *current_memo = res;
    res
}