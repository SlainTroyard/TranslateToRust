use std::io::{self, BufRead};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap() as usize;

    // Count the frequency of each number in nums
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    // Calculate the number of pairs with GCD equal to each number from mx to 1
    let mut cnt_gcd = vec![0; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c * (c - 1) / 2) as i64;
    }

    // Calculate the cumulative sum of cnt_gcd
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Process queries
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx as i32;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read nums
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(lines.next().unwrap()?.trim().parse().unwrap());
    }

    // Read queries
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        queries.push(lines.next().unwrap()?.trim().parse().unwrap());
    }

    // Calculate and print results
    let ans = gcd_values(&nums, &queries);
    for &result in &ans {
        print!("{} ", result);
    }
    println!();

    Ok(())
}