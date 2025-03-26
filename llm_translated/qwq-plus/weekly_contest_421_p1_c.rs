use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % temp;
        a = temp;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

pub fn max_score(nums: &[i32]) -> i64 {
    let n = nums.len();
    if n == 1 {
        return (nums[0] as i64).pow(2);
    }

    let mut lcms = vec![0; n];
    let mut gcds = vec![0; n];
    lcms[n - 1] = nums[n - 1] as i64;
    gcds[n - 1] = nums[n - 1] as i64;

    // Compute lcms and gcds from the end backwards
    for i in (0..n - 1).rev() {
        let a = nums[i] as i64;
        let b_lcm = lcms[i + 1];
        let b_gcd = gcds[i + 1];
        lcms[i] = lcm(a, b_lcm);
        gcds[i] = gcd(a, b_gcd);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    // Track prelcm and pregcd from the start forwards
    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;

    // Iterate from 1 to n-2 (inclusive)
    for i in 1..(n - 1) {
        let current_gcd = gcd(pregcd, gcds[i + 1]);
        let current_lcm = lcm(prelcm, lcms[i + 1]);
        ans = ans.max(current_gcd * current_lcm);

        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }

    ans = ans.max(prelcm * pregcd);
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let nums_size: usize = iter.next().unwrap().parse()
        .expect("Invalid numsSize");
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| iter.next().expect("Not enough numbers"))
        .map(|x| x.parse().unwrap())
        .collect();
    let result = max_score(&nums);
    println!("{}", result);
}