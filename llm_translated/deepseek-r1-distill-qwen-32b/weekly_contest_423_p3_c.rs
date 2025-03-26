fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut cnt = vec![0i64; 100003];
    let mut sum = vec![0i64; 100003];
    let mut ans = 0i64;

    for &num in nums {
        let x = (num + 1) as usize;
        let x = std::cmp::min(x, 100002);

        let x_minus_1 = if x > 0 { x - 1 } else { 0 };
        let x_plus_1 = if x < 100002 { x + 1 } else { 100002 };

        let c = (if x > 0 { cnt[x_minus_1] } else { 0 } + 1 + if x < 100002 { cnt[x_plus_1] } else { 0 }) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;

        let s = (c * (x as i64 - 1) + if x > 0 { sum[x_minus_1] } else { 0 } + if x < 100002 { sum[x_plus_1] } else { 0 }) % MOD;
        sum[x] = (sum[x] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans as i32
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap() as usize;
    let nums: Vec<i32> = iter.take(n).collect();
    let result = sum_of_good_subsequences(&nums);
    println!("{}", result);
}