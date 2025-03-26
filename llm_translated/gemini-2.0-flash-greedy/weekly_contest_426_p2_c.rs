use std::io;
use std::io::Read;

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    let mut set = [0; 2001];
    for &num in nums {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 {
            // curr_sum is even
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if half >= -1000 && half <= 1000 {
                if set[(half + 1000) as usize] > threshold {
                    if ans < num {
                        ans = num;
                    }
                }
            }
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse()?;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if nums.len() != nums_size {
        panic!("Incorrect number of elements provided");
    }

    let result = get_largest_outlier(&nums);

    println!("{}", result);

    Ok(())
}