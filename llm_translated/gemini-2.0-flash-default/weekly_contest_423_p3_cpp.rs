use std::collections::HashMap;
use std::io;
use std::io::Read;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut sum: HashMap<i32, i64> = HashMap::new();

    for &i in nums {
        let i_i64 = i as i64;
        let prev = (i - 1) as i32;
        let next = (i + 1) as i32;

        let cnt_prev = *cnt.get(&prev).unwrap_or(&0);
        let cnt_next = *cnt.get(&next).unwrap_or(&0);

        let sum_prev = *sum.get(&prev).unwrap_or(&0);
        let sum_next = *sum.get(&next).unwrap_or(&0);

        let new_cnt = (cnt_prev + cnt_next + 1) % MOD;

        *cnt.entry(i).or_insert(0) += new_cnt;
        *cnt.entry(i).or_insert(0) %= MOD;

        *sum.entry(i).or_insert(0) += (sum_prev + sum_next) % MOD;
        *sum.entry(i).or_insert(0) %= MOD;

        *sum.entry(i).or_insert(0) += ((cnt_prev + cnt_next + 1) % MOD) * i_i64 % MOD;
        *sum.entry(i).or_insert(0) %= MOD;

    }

    let mut res: i64 = 0;
    for &val in sum.values() {
        res += val;
        res %= MOD;
    }

    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse()?;
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = sum_of_good_subsequences(&nums);

    println!("{}", result);

    Ok(())
}