use std::io::{self, Read};

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    
    // Create frequency map (equivalent to 'set' in the C code)
    let mut frequency = vec![0; 2001];
    for &num in nums {
        frequency[(num + 1000) as usize] += 1;
    }
    
    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        if (curr_sum & 1) == 0 {  // curr_sum is even
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };
            
            if half >= -1000 && half <= 1000 {
                if frequency[(half + 1000) as usize] > threshold {
                    if ans < num {
                        ans = num;
                    }
                }
            }
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut values = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Expected a number"));
    
    // Read the size of the array
    let nums_size = values.next().unwrap() as usize;
    
    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        if let Some(num) = values.next() {
            nums.push(num);
        }
    }
    
    // Calculate the largest outlier
    let result = get_largest_outlier(&nums);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}