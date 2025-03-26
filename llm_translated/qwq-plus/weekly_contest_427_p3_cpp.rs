use std::io;

struct Solution;

impl Solution {
    pub fn max_subarray_sum(v: Vec<i32>, k: usize) -> i64 {
        let mut m = vec![i64::MAX; k];
        let mut ans = i64::MIN;
        let mut sm = 0;
        for (i, &num) in v.iter().enumerate() {
            sm += num as i64;
            let cur_sz = i + 1;
            if cur_sz % k == 0 {
                ans = ans.max(sm);
            }
            let y = cur_sz % k;
            if m[y] != i64::MAX {
                ans = ans.max(sm - m[y]);
                m[y] = m[y].min(sm);
            } else {
                m[y] = sm;
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut nums = Vec::new();
    for _ in 0..n {
        let num: i32 = tokens.next().unwrap().parse().unwrap();
        nums.push(num);
    }
    let result = Solution::max_subarray_sum(nums, k);
    println!("{}", result);
}