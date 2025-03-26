fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let mut r: usize = 1;
    let nums_size = nums.len();
    while r < nums_size {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() {
    use std::io;

    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input for numsSize");

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for num");
        nums.push(num);
    }

    let result = find_maximum_score(&nums);
    println!("{}", result);
}