use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Read the dimensions of the grid and the value of k
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    // Read the limits
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let limits: Vec<usize> = (0..n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

    // Calculate the maximum sum
    let result = max_sum(&mut grid, &limits, k);
    println!("{}", result);
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &[usize], k: usize) -> i64 {
    let n = grid.len();
    let m = grid[0].len();

    // Sort each row in descending order
    for row in grid.iter_mut() {
        row.sort_by(|a, b| b.cmp(a));
    }

    // Use a max-heap to keep track of the largest elements
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((grid[i][0], i, 0));
    }

    let mut ans: i64 = 0;
    let mut remaining = k;

    while remaining > 0 && !heap.is_empty() {
        let (val, r, c) = heap.pop().unwrap();
        if c >= limits[r] {
            continue;
        }
        ans += val as i64;
        remaining -= 1;
        if c + 1 < m {
            heap.push((grid[r][c + 1], r, c + 1));
        }
    }

    ans
}