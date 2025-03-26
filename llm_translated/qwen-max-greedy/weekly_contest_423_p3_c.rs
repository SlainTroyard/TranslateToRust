use std::io;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i64 {
    let mut cnt = vec![0; 100003];
    let mut sum = vec![0; 100003];
    let mut ans: i64 = 0;

    for i in 0..nums.len() {
        let x = (nums[i] as usize + 1) % 100003;
        nums[i] += 1;
        let c = (cnt[x - 1] + 1 + cnt[(x + 1) % 100003]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[(x + 1) % 100003]) % MOD;
        sum[x] = (sum[x] + s) % MOD;
        ans = (ans + s) % MOD;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    // Dynamically allocate memory for the array
    let mut nums: Vec<i32> = vec![0; n];

    // Read the elements of the array
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        nums[i] = input.trim().parse()?;
    }

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);

    Ok(())
}