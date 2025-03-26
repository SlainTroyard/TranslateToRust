// Helper functions to replicate the original C code's min and max
fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn max(a: i32, b: i32, c: i32) -> i32 {
    a.max(b).max(c)
}

fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let mut maxlen = 0;
    let mut i = 1;
    let mut max1 = 1;

    while i < nums.len() {
        let mut max2 = 1;
        // Find the current increasing subarray
        while i < nums.len() && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2);
        maxlen = max(maxlen, temp, max2 / 2);
        max1 = max2;
        i += 1;
    }

    maxlen
}

fn read_input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    input.split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect()
}

fn main() {
    let input = read_input();
    if input.is_empty() {
        panic!("No input provided");
    }
    let nums_size = input[0] as usize;
    if input.len() < 1 + nums_size {
        panic!("Not enough numbers provided");
    }
    let nums = &input[1..1 + nums_size];
    let result = max_increasing_subarrays(nums);
    println!("{}", result);
}