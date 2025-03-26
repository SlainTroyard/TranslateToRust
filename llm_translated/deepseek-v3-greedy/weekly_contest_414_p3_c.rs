use std::io::{self, Write};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut l = 0usize;
    let mut r = 1usize;

    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }

    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input.trim().parse().expect("Invalid input for numsSize");

    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input for num");
        nums.push(num);
    }

    let result = find_maximum_score(&nums);
    println!("{}", result);
}