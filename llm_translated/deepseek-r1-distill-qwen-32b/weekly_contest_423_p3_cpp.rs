use std::collections::HashMap;
use std::io;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: Vec<i64>, mod: i64) -> i64 {
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    let mut sum: HashMap<i64, i64> = HashMap::new();
    
    for num in nums {
        // Update cnt
        let cnt_add = ( *cnt.get(&(num - 1)).unwrap_or(&0) + *cnt.get(&(num + 1)).unwrap_or(&0) + 1 ) % mod;
        let current_cnt = *cnt.get(&num).unwrap_or(&0);
        let new_cnt = (current_cnt + cnt_add) % mod;
        cnt.insert(num, new_cnt);
        
        // Update sum
        let sum_add1 = ( *sum.get(&(num - 1)).unwrap_or(&0) + *sum.get(&(num + 1)).unwrap_or(&0) ) % mod;
        let cnt_part = ( *cnt.get(&(num - 1)).unwrap_or(&0) + *cnt.get(&(num + 1)).unwrap_or(&0) + 1 ) % mod;
        let sum_add2 = (cnt_part * num) % mod;
        let current_sum = *sum.get(&num).unwrap_or(&0);
        let new_sum = (current_sum + sum_add1 + sum_add2) % mod;
        sum.insert(num, new_sum);
    }
    
    // Calculate result
    let mut res = 0;
    for s in sum.values() {
        res = (res + s) % mod;
    }
    res
}

fn main() {
    let mod = MOD;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    
    let n = tokens.next().unwrap();
    let nums: Vec<i64> = tokens.take(n as usize).collect();
    
    let result = sum_of_good_subsequences(nums, mod);
    println!("{}", result);
}