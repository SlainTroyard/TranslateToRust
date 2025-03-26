use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let k: usize = input.trim().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    for num in input.trim().split_whitespace() {
        nums.push(num.parse::<i64>().unwrap());
    }

    let result = max_subarray_sum(nums, k);
    println!("{}", result);
    Ok(())
}

fn max_subarray_sum(nums: Vec<i64>, k: usize) -> i64 {
    let mut prefix_sums = HashMap::new();
    let mut ans = i64::MIN;
    let mut sm = 0;

    for i in 0..nums.len() {
        sm += nums[i];
        let cur_sz = i + 1;

        // Check if current subarray length is exactly divisible by k
        if cur_sz % k == 0 {
            if sm > ans {
                ans = sm;
            }
        }

        let y = cur_sz % k;
        match prefix_sums.get_mut(&y) {
            Some(&mut val) => {
                if sm - val > ans {
                    ans = sm - val;
                }
                if sm < val {
                    *val = sm;
                }
            }
            None => {
                prefix_sums.insert(y, sm);
            }
        }
    }

    ans
}