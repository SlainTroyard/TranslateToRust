use std::io;

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    let mut frequency = [0; 2001];
    for &num in nums {
        frequency[(num + 1000) as usize] += 1;
    }

    let mut largest_outlier = -1001;

    for &num in nums {
        let current_sum = total_sum - num;
        if (current_sum & 1) == 0 {
            let half = current_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if half >= -1000 && half <= 1000 {
                let index = (half + 1000) as usize;
                if frequency[index] > threshold {
                    if num > largest_outlier {
                        largest_outlier = num;
                    }
                }
            }
        }
    }

    largest_outlier
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    let nums_size: i32 = tokens.next().unwrap().parse().expect("Invalid size");
    if nums_size < 0 {
        eprintln!("Size cannot be negative");
        std::process::exit(1);
    }
    let nums_size = nums_size as usize;

    let nums: Vec<i32> = tokens
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    if nums.len() != nums_size {
        eprintln!("Number of elements does not match size");
        std::process::exit(1);
    }

    let result = get_largest_outlier(&nums);
    println!("{}", result);
}