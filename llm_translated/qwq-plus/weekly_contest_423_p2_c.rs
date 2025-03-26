use std::io;

fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let mut maxlen = 0;
    let mut i = 1;
    let mut max1 = 1;
    let nums_size = nums.len();

    while i < nums_size {
        let mut max2 = 1;
        while i < nums_size && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = std::cmp::min(max1, max2);
        let current_max = std::cmp::max(temp, max2 / 2);
        maxlen = std::cmp::max(maxlen, current_max);
        max1 = max2;
        i += 1;
    }
    maxlen
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("Failed to read input");

    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        println!("0");
        return;
    }

    let nums_size = tokens[0].parse::<i32>().unwrap();
    let nums: Vec<i32> = tokens[1..=(nums_size as usize)]
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
}