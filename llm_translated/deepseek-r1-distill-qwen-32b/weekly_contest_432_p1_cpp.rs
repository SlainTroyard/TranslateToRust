use std::io;

fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    if n == 0 {
        return Vec::new();
    }
    let m = grid[0].len();
    let mut result = Vec::new();
    
    for i in 0..n {
        let row = &grid[i];
        if i % 2 == 0 {
            // Even row: left to right, step by 2
            for j in (0..m).step_by(2) {
                result.push(row[j]);
            }
        } else {
            // Odd row: right to left, step by 2
            for j in (0..m).rev().step_by(2) {
                result.push(row[j]);
            }
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    if tokens.len() < 2 {
        println!();
        return Ok(());
    }
    
    let gridSize = tokens[0] as usize;
    let gridColSize = tokens[1] as usize;
    let data = &tokens[2..];
    
    let mut grid = Vec::new();
    for i in 0..gridSize {
        let start = i * gridColSize;
        let end = start + gridColSize;
        let row = data[start..end].to_vec();
        grid.push(row);
    }
    
    let ans = zigzag_traversal(grid);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    
    Ok(())
}