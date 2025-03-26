use std::io::Read;

struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn multiply(a: &[[i32; 26]; 26], b: &[[i32; 26]; 26]) -> [[i32; 26]; 26] {
        let mut result = [[0; 26]; 26];
        for i in 0..26 {
            for k in 0..26 {
                let a_ik = a[i][k];
                if a_ik == 0 {
                    continue;
                }
                for j in 0..26 {
                    let product = a_ik as i64 * b[k][j] as i64;
                    let sum = result[i][j] as i64 + product;
                    result[i][j] = (sum % Solution::MOD) as i32;
                }
            }
        }
        result
    }

    fn matrix_pow(mut a: [[i32; 26]; 26], mut n: i32) -> [[i32; 26]; 26] {
        let mut res = [[0; 26]; 26];
        // Initialize identity matrix
        for i in 0..26 {
            res[i][i] = 1;
        }
        while n != 0 {
            if n & 1 != 0 {
                res = Solution::multiply(&res, &a);
            }
            a = Solution::multiply(&a, &a);
            n >>= 1;
        }
        res
    }

    pub fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
        let mut m = [[0; 26]; 26];
        for i in 0..26 {
            let num = nums[i];
            if num == 0 {
                continue;
            }
            let start = i as i32 + 1;
            let end = i as i32 + num;
            for j in start..=end {
                let idx = j.rem_euclid(26) as usize;
                m[i][idx] = 1;
            }
        }
        let m_pow = Solution::matrix_pow(m, t);

        let mut cnt = [0i64; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
        }

        let mut ans = 0i64;
        for i in 0..26 {
            let row_sum: i32 = m_pow[i].iter().sum();
            ans += row_sum as i64 * cnt[i];
            ans %= Solution::MOD;
        }
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read and ignore the string length (not used)
    let _s_len = tokens.next().unwrap().parse::<usize>().unwrap();
    let s = tokens.next().unwrap();
    let t = tokens.next().unwrap().parse::<i32>().unwrap();

    let mut nums = Vec::with_capacity(26);
    for _ in 0..26 {
        nums.push(tokens.next().unwrap().parse::<i32>().unwrap());
    }

    let result = Solution::length_after_transformations(s, t, &nums);
    println!("{}", result);
}