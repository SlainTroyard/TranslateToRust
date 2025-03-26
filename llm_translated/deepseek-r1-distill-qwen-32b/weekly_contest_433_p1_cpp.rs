struct Solution;

impl Solution {
    pub fn subarray_sum(&self, nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }
        let mut ans = 0;
        for i in 0..n {
            let current = nums[i];
            let start = if current > i as i32 {
                0
            } else {
                i as i32 - current
            };
            let start = start.max(0) as usize;
            ans += prefix_sum[i + 1] - prefix_sum[start];
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let n = tokens[0].parse::<i32>().expect("Invalid n");
    let nums: Vec<i32> = tokens[1..(n as usize + 1)]
        .iter()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    let solution = Solution;
    println!("{}", solution.subarray_sum(&nums));
}