use std::io;

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    for i in 0..n {
        let len = n - i;
        let mut diag = Vec::with_capacity(len);
        for x in 0..len {
            diag.push(grid[i + x][x]);
        }
        diag.sort();
        diag.reverse();
        for x in 0..len {
            grid[i + x][x] = diag[x];
        }
    }
    for i in 1..n {
        let len = n - i;
        let mut diag = Vec::with_capacity(len);
        for x in 0..len {
            diag.push(grid[x][i + x]);
        }
        diag.sort();
        for x in 0..len {
            grid[x][i + x] = diag[x];
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = tokens[0] as usize;
    let grid_size = n * n;
    assert_eq!(tokens.len(), 1 + grid_size, "Input has incorrect number of elements");
    
    let mut grid = vec![vec![0; n]; n];
    let mut idx = 1;
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = tokens[idx];
            idx += 1;
        }
    }
    
    sort_matrix(&mut grid);
    
    for row in &grid {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}