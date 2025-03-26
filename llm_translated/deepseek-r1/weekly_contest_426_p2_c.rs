use std::io::{self, Read};

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut set = [0; 2001];
    
    // Populate frequency set, offsetting by 1000 to handle negative indices
    for &num in nums {
        let index = (num + 1000) as usize;
        set[index] += 1;
    }

    let mut ans = -1001;

    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 {
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            // Check if half is within valid range and exists in sufficient quantity
            if (-1000..=1000).contains(&half) {
                let count = set[(half + 1000) as usize];
                if count > threshold && num > ans {
                    ans = num;
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid number"));

    // Read array size and elements following original C code's input handling
    let nums_size = tokens.next().expect("No size provided");
    let nums: Vec<i32> = tokens.take(nums_size as usize).collect();

    // Process and print result
    let result = get_largest_outlier(&nums);
    println!("{}", result);
}