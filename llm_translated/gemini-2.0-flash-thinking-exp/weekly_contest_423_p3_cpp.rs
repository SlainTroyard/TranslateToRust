use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut sum: HashMap<i32, i64> = HashMap::new();

    for &i in nums {
        let i_i64 = i as i64;

        let prev = cnt.get(&(i - 1)).copied().unwrap_or(0);
        let next = cnt.get(&(i + 1)).copied().unwrap_or(0);

        let new_cnt = (prev + next + 1) % MOD;
        cnt.entry(i).or_insert(0);
        *cnt.get_mut(&i).unwrap() = (*cnt.get(&i).unwrap() + new_cnt) % MOD;
        

        let prev_sum = sum.get(&(i - 1)).copied().unwrap_or(0);
        let next_sum = sum.get(&(i + 1)).copied().unwrap_or(0);

        sum.entry(i).or_insert(0);
        *sum.get_mut(&i).unwrap() = (*sum.get(&i).unwrap() + (prev_sum + next_sum) % MOD) % MOD;

        let add_sum = ((cnt.get(&(i - 1)).copied().unwrap_or(0) + cnt.get(&(i + 1)).copied().unwrap_or(0) + 1) % MOD) * i_i64 % MOD;
        *sum.get_mut(&i).unwrap() = (*sum.get(&i).unwrap() + add_sum) % MOD;
        
    }

    let mut res: i64 = 0;
    for &value in sum.values() {
        res = (res + value) % MOD;
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: usize = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let nums: Vec<i32> = iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = sum_of_good_subsequences(&nums);

    println!("{}", result);
}