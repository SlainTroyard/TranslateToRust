use std::io::Read;

fn max_sum(grid: &[Vec<i32>], limits: &[i32], k: usize) -> i64 {
    // Collect the top 'limits[i]' elements from each sorted row into a list
    let mut lst = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut sorted_row = row.clone();
        sorted_row.sort_by(|a, b| b.cmp(a)); // Sort row in descending order
        let take_count = limits[i] as usize;
        lst.extend(&sorted_row[..take_count]); // Take first 'limits[i]' elements
    }
    
    // Sort the combined list and sum the top k elements
    lst.sort_by(|a, b| b.cmp(a));
    lst.iter()
        .take(k)
        .map(|&num| num as i64)
        .sum()
}

fn main() {
    // Read entire input and split into tokens
    let input = {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    };
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Parse n (rows), m (columns), and k (target elements)
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let k = tokens.next().unwrap();

    // Read grid data (n rows × m columns)
    let mut grid = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut row = Vec::with_capacity(m as usize);
        for _ in 0..m {
            row.push(tokens.next().unwrap());
        }
        grid.push(row);
    }

    // Read limits (one per row)
    let mut limits = Vec::with_capacity(n as usize);
    for _ in 0..n {
        limits.push(tokens.next().unwrap());
    }

    // Calculate and print result
    let result = max_sum(&grid, &limits, k as usize);
    println!("{}", result);
}