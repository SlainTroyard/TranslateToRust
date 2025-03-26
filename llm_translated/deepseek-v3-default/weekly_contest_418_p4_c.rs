use std::io::{self, Write};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap_or(&0) as usize;

    // Count the occurrences of each number
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    // Calculate the GCD counts
    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c * (c - 1) / 2) as i64;
    }

    // Accumulate the GCD counts
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Answer the queries using binary search
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left as i32);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    let mut nums = Vec::with_capacity(nums_size);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        nums.push(num.parse().unwrap());
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let queries_size: usize = input.trim().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for query in input.split_whitespace() {
        queries.push(query.parse().unwrap());
    }

    let ans = gcd_values(&nums, &queries);
    for val in ans {
        print!("{} ", val);
    }
    println!();
}