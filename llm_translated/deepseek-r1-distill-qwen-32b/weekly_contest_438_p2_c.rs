fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<usize>, k: usize) -> i64 {
    let mut all_elements = Vec::new();

    for i in 0..grid.len() {
        let mut row = grid[i].clone();
        row.sort_unstable_descending();
        let limit = limits[i];
        all_elements.extend_from_slice(&row[..limit]);
    }

    all_elements.sort_unstable_descending();

    let sum: i64 = all_elements.iter()
        .take(k)
        .map(|&x| x as i64)
        .sum();

    sum
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid integer"));

    let n = tokens.next().unwrap() as usize;
    let m = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap() as usize;

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = tokens.take(m).collect();
        grid.push(row);
    }

    let limits: Vec<usize> = tokens.take(n).map(|x| x as usize).collect();

    let result = max_sum(&grid, &limits, k);
    println!("{}", result);
}