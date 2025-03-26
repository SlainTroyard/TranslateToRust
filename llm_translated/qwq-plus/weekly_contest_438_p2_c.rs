use std::io;

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &[usize], k: usize) -> i64 {
    let mut all_elements = Vec::new();
    for (row, &limit) in grid.iter_mut().zip(limits.iter()) {
        // Sort the row in descending order
        row.sort_unstable_by(|a, b| b.cmp(a));
        // Collect the first 'limit' elements
        all_elements.extend_from_slice(&row[0..limit]);
    }
    // Sort all elements in descending order again
    all_elements.sort_unstable_by(|a, b| b.cmp(a));
    // Sum the first k elements
    all_elements.iter().take(k).map(|&x| x as i64).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let mut grid = vec![Vec::new(); n];
    for row in &mut grid {
        for _ in 0..m {
            row.push(tokens.next().unwrap().parse().unwrap());
        }
    }

    let mut limits = Vec::with_capacity(n);
    for _ in 0..n {
        limits.push(tokens.next().unwrap().parse().unwrap());
    }

    let result = max_sum(&mut grid, &limits, k);
    println!("{}", result);
}