use std::io::{self, BufRead};

fn cmp_asc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn cmp_desc(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
}

fn sort_matrix(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    
    for i in 0..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in i..n {
            a.push(grid[k][k - i]);
        }
        a.sort_by(cmp_desc);
        for k in i..n {
            grid[k][k - i] = a[k - i];
        }
    }
    
    for i in 1..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in 0..n {
            a.push(grid[k][k + i]);
        }
        a.sort_by(cmp_asc);
        for k in 0..n {
            grid[k][k + i] = a[k];
        }
    }
    
    grid.clone()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Sort the matrix
    let result = sort_matrix(&mut grid);

    // Print the result
    for row in result {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }

    Ok(())
}