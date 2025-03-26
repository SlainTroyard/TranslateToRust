use std::io;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i64 {
    let mut cnt = vec![0; 100003];
    let mut sum = vec![0; 100003];
    let mut ans: i64 = 0;

    for &mut x in nums.iter_mut() {
        x += 1;
        let c = (cnt[x as usize] + (cnt[(x - 1) as usize] + 1 + cnt[(x + 1) as usize]) % MOD) % MOD;
        cnt[x as usize] = c as i32;
        let s = (c * (x - 1) as i64 + sum[(x - 1) as usize] + sum[(x + 1) as usize]) % MOD;
        sum[x as usize] = s as i32;
        ans = (ans + s) % MOD;
    }

    ans
}

fn main() {
    // Read the size of the array
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Dynamically allocate memory for the array
    let mut nums: Vec<i32> = vec![0; n];

    // Read the elements of the array
    for i in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        nums[i] = num.trim().parse().expect("Please type a number!");
    }

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}