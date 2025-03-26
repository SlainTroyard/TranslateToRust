use std::io::{self, BufRead};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut cnt_x = vec![0; mx + 1];
    for &x in nums {
        cnt_x[x as usize] += 1;
    }

    let mut cnt_gcd = vec![0; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c * (c - 1) / 2) as i64;
    }
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    queries.iter().map(|&q| {
        cnt_gcd.binary_search(&q).unwrap_err() as i32
    }).collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read nums
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read queries
    let q: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let queries: Vec<i64> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print result
    let ans = gcd_values(&nums, &queries);
    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}