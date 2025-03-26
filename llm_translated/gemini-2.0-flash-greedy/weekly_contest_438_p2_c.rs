use std::cmp::Reverse;
use std::io;

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
    let mut len = 0;
    for &limit in limits {
        len += limit;
    }

    let mut lst: Vec<i32> = Vec::with_capacity(len as usize);

    for (i, row) in grid.iter_mut().enumerate() {
        row.sort_by_key(|&x| Reverse(x));
        for j in 0..limits[i] {
            lst.push(row[j as usize]);
        }
    }

    lst.sort_by_key(|&x| Reverse(x));

    let mut ans: i64 = 0;
    for i in 0..k as usize {
        ans += lst[i] as i64;
    }
    ans
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let mut limits: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let limit: i32 = line.trim().parse().unwrap();
        limits.push(limit);
    }

    println!("{}", max_sum(&mut grid, &limits, k));
}