use std::io;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}

pub fn max_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut suf_gcd = vec![0; n + 1];
    let mut suf_lcm = vec![0; n + 1];
    suf_lcm[n] = 1;
    for i in (0..n).rev() {
        suf_gcd[i] = gcd(suf_gcd[i + 1] as i64, nums[i] as i64) as i32;
        suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
    }

    let mut ans = (suf_gcd[0] as i64) * suf_lcm[0];
    let mut pre_gcd = 0;
    let mut pre_lcm = 1;

    for i in 0..n {
        let current_gcd = gcd(pre_gcd as i64, suf_gcd[i + 1] as i64);
        let current_lcm = lcm(pre_lcm, suf_lcm[i + 1]);
        ans = ans.max(current_gcd * current_lcm);

        let new_gcd = gcd(pre_gcd as i64, nums[i] as i64);
        pre_gcd = new_gcd as i32;
        let new_lcm = lcm(pre_lcm, nums[i] as i64);
        pre_lcm = new_lcm;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let n = tokens[0] as usize;
    let nums = &tokens[1..=n];

    let result = max_score(nums.to_vec());
    println!("{}", result);
}