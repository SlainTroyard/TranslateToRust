use std::io;

struct Solution;

impl Solution {
    fn gcd_values(&self, nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let mx = *nums.iter().max().unwrap() as usize;
        let mut cnt_x = vec![0; mx + 1];
        for &x in &nums {
            cnt_x[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0i64; mx + 1];
        for i in (1..=mx).rev() {
            let mut c = 0;
            let mut sum_gcd = 0;
            for j in (i..=mx).step_by(i) {
                c += cnt_x[j];
                sum_gcd += cnt_gcd[j];
            }
            cnt_gcd[i] = (c as i64 * (c as i64 - 1) / 2) - sum_gcd;
        }

        // Compute prefix sum
        let mut sum = 0i64;
        for i in 0..=mx {
            sum += cnt_gcd[i];
            cnt_gcd[i] = sum;
        }

        // For each query, find the upper bound
        let mut ans = Vec::with_capacity(queries.len());
        for &q in &queries {
            let idx = cnt_gcd.partition_point(|&x| x <= q);
            ans.push(idx as i32);
        }

        ans
    }
}

fn main() {
    let input = io::stdin().read_to_string().unwrap();
    let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();

    let mut ptr = 0;
    let n: i32 = tokens[ptr].parse().unwrap();
    ptr += 1;
    let nums: Vec<i32> = tokens[ptr..ptr + n as usize].iter().map(|s| s.parse().unwrap()).collect();
    ptr += n as usize;
    let q: i32 = tokens[ptr].parse().unwrap();
    ptr += 1;
    let queries: Vec<i64> = tokens[ptr..ptr + q as usize].iter().map(|s| s.parse().unwrap()).collect();

    let solution = Solution {};
    let ans = solution.gcd_values(nums, queries);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}