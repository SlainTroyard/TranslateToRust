use std::io;
use std::io::Read;
use std::cmp::max;

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();

    let mut mx = 0;
    for &num in nums {
        mx = max(mx, num);
    }

    let mx = mx as usize;

    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c as i64) * ((c - 1) as i64) / 2;
    }

    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    let mut ans = Vec::with_capacity(queries_size);
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let queries: Vec<i64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let ans = gcd_values(&nums, &queries);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}