use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut ans = 0i64;
    let mut cnt = vec![0i64; 100003];
    let mut sum = vec![0i64; 100003];
    
    for num in nums.iter_mut() {
        *num += 1;
        let x = *num as usize;
        
        // Calculate c using previous cnt values
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;
        
        // Calculate s and update sum[x]
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;
        
        // Accumulate the answer
        ans = (ans + s) % MOD;
    }
    
    ans as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Read the size of the array
    let n = match tokens.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(val) => val,
        None => {
            eprintln!("Invalid input for array size");
            return;
        }
    };
    
    // Read the array elements
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        match tokens.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(num) => nums.push(num),
            None => {
                eprintln!("Invalid input for array element");
                return;
            }
        }
    }
    
    // Process and print the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}