use std::io;
use std::str::SplitWhitespace;

fn read_line_and_parse_ints() -> Result<Vec<i32>, String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
    let parts: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(parts)
}

fn read_line_to_vec_i32() -> Result<Vec<i32>, String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
    line.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}

fn read_n_lines_to_vec_vec_i32(n: usize, m: usize) -> Result<Vec<Vec<i32>>, String> {
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row_str = read_line_to_vec_i32()?;
        if row_str.len() != m {
            return Err("Input grid column size mismatch".to_string());
        }
        grid.push(row_str);
    }
    Ok(grid)
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
    let mut len = 0;
    for limit in limits {
        len += limit;
    }
    let mut lst: Vec<i32> = Vec::with_capacity(len as usize);
    for i in 0..grid.len() {
        grid[i].sort_by(|a, b| b.cmp(a)); // Sort each row in descending order
        for j in 0..limits[i] {
            lst.push(grid[i][j as usize]);
        }
    }
    lst.sort_by(|a, b| b.cmp(a)); // Sort lst in descending order
    let mut ans: i64 = 0;
    for i in 0..(k as usize).min(lst.len()) {
        ans += lst[i] as i64;
    }
    ans
}

fn main() -> Result<(), String> {
    let first_line = read_line_and_parse_ints()?;
    if first_line.len() != 3 {
        return Err("Input format for n, m, k is incorrect".to_string());
    }
    let n = first_line[0] as usize;
    let m = first_line[1] as usize;
    let k = first_line[2];

    let mut grid = read_n_lines_to_vec_vec_i32(n, m)?;
    let limits = read_line_to_vec_i32()?;
    if limits.len() != n {
        return Err("Input limits size mismatch".to_string());
    }

    let result = max_sum(&mut grid, &limits, k);
    println!("{}", result);
    Ok(())
}