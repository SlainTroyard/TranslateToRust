use std::io;
use std::vec;

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mut mx = 0;
    for &num in nums {
        if num > mx {
            mx = num;
        }
    }

    let mut cnt_x = vec![0; (mx + 1) as usize];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    let mut cnt_gcd = vec![0i64; (mx + 1) as usize];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i as usize) {
            c += cnt_x[j as usize];
            cnt_gcd[i as usize] -= cnt_gcd[j as usize];
        }
        cnt_gcd[i as usize] += (c as i64) * (c as i64 - 1) / 2;
    }

    for i in 1..=mx {
        cnt_gcd[i as usize] += cnt_gcd[(i - 1) as usize];
    }

    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid as usize] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left);
    }
    ans
}

fn main() {
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).unwrap();
    let nums_size: i32 = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut queries_size_str = String::new();
    io::stdin().read_line(&mut queries_size_str).unwrap();
    let queries_size: i32 = queries_size_str.trim().parse().unwrap();

    let mut queries_str = String::new();
    io::stdin().read_line(&mut queries_str).unwrap();
    let queries: Vec<i64> = queries_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let ans = gcd_values(&nums, &queries);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}