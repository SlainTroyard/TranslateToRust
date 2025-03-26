use std::io::{self, Read};

const MOD: i64 = 1000000007;
// The C code declares arrays of size 100003 for cnt and sum.
const MAX: usize = 100003;

/// Computes the sum of good subsequences as described in the problem.
/// 
/// The algorithm follows the logic from the C code:
/// - For each number in nums, we first increment it (similar to ++nums[i]).
/// - Let x be the updated number. Then we compute:
///     c = cnt[x-1] + 1 + cnt[x+1]
///     cnt[x] = (cnt[x] + c) mod MOD
///     s = c*(x-1) + sum[x-1] + sum[x+1]
///     sum[x] = (sum[x] + s) mod MOD
/// - We accumulate s into the answer and finally return it modulo MOD.
fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt = vec![0i64; MAX];
    let mut sum = vec![0i64; MAX];
    let mut ans = 0i64;
    
    // Iterate over each number in the array.
    // The C code did: x = ++nums[i], so here we increment in place.
    for num in nums.iter_mut() {
        *num += 1; // Increment the number (like ++nums[i])
        let x = *num as usize; // Use the updated value as index
        
        // Compute c = cnt[x-1] + 1 + cnt[x+1] (using modulo arithmetic)
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        cnt[x] = (cnt[x] + c) % MOD;
        
        // Compute s = c * (x - 1) + sum[x-1] + sum[x+1] (again modulo MOD)
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        sum[x] = (sum[x] + s) % MOD;
        
        // Accumulate s into the answer
        ans = (ans + s) % MOD;
    }
    
    ans as i32
}

fn main() {
    // Read entire input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    // Split input by whitespace to parse tokens as in the C code's scanf logic.
    let mut tokens = input.split_whitespace();

    // Read the size of the array.
    let n: usize = tokens
        .next()
        .expect("Expected the array size")
        .parse()
        .expect("Failed to parse the array size");

    // Dynamically allocate and read the elements of the array.
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = tokens
            .next()
            .expect("Expected a number for the array element")
            .parse()
            .expect("Failed to parse an array element");
        nums.push(value);
    }

    // Compute the result using the algorithm.
    let result = sum_of_good_subsequences(&mut nums);

    // Print the result to stdout (matching the exact output format of the C code).
    println!("{}", result);
}