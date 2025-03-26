use std::io;

/// Computes the maximum score based on the given algorithm.
///
/// The algorithm tracks the left index `l` and iterates with `r` to find the next higher element.
/// When a higher element is found at `r`, it adds the contribution from `l` to `r-1` to the result
/// and updates `l` to `r`. After the loop, it adds the contribution from the last segment.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l = 0usize;
    let mut r = 1usize;
    let n = nums.len();
    while r < n {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    ans + ((r - l - 1) as i64) * nums[l] as i64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let nums_size = parts.next().unwrap() as usize;
    let nums: Vec<i32> = parts.take(nums_size).collect();
    let ans = find_maximum_score(&nums);
    println!("{}", ans);
}