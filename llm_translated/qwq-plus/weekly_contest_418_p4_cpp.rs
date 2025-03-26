use std::io;

struct Solution;

impl Solution {
    fn gcd_values(&self, nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut cnt_x = vec![0; max_num + 1];
        for &num in &nums {
            cnt_x[num as usize] += 1;
        }

        let mut cnt_gcd = vec![0i64; max_num + 1];
        for i in (1..=max_num).rev() {
            let mut c = 0i64;
            for j in (i..=max_num).step_by(i) {
                c += cnt_x[j] as i64;
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += c * (c - 1) / 2;
        }

        // Compute prefix sums
        for i in 1..=max_num {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        let mut ans = Vec::with_capacity(queries.len());
        for &q in &queries {
            let target = q;
            let pos = match cnt_gcd.binary_search(&target) {
                Ok(idx) => idx + 1,
                Err(idx) => idx,
            };
            ans.push(pos as i32);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens.next().unwrap().parse().unwrap();
        nums.push(num);
    }

    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let query: i64 = tokens.next().unwrap().parse().unwrap();
        queries.push(query);
    }

    let solution = Solution {};
    let ans = solution.gcd_values(nums, queries);

    for &x in &ans {
        print!("{} ", x);
    }
    println!();
}