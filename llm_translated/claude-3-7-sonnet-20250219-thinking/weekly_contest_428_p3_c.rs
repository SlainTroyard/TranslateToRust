use std::io::{self, Read};
use std::cmp;

fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; n];
    let mut kmpcnt = 0;
    
    // First phase - process the whole array
    for i in 1..n {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 == 1 && ((i + 1) / 2) % (i + 1 - kmpcnt) == 0 {
            res += (n as i32) - (i as i32) - 1;
        }
    }
    
    // Second phase - process subarrays
    for i in 1..n {
        let mut cnt = vec![0; n - i];
        let mut end = n;
        kmpcnt = 0;
        
        if 2 * i < n && (i % (2 * i - cnt0[2 * i - 1]) == 0) {
            end = cmp::min(end, 3 * i);
        }
        
        for j in (i + 1)..end {
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            cnt[j - i] = kmpcnt;
            
            if (j - i + 1) % 2 == 0 && ((j - i + 1) / 2) % (j - i + 1 - kmpcnt) == 0 {
                let len = j - i + 1;
                let div = i - 1 + len / 2;
                
                if len == i * 2 && (i % (div + 1 - cnt0[div]) == 0) {
                    break;
                }
                
                res += 1;
            }
        }
    }
    
    res
}

fn main() {
    // Read size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input: expected an integer");
    
    // Read array elements
    let mut nums = Vec::with_capacity(n);
    let mut i = 0;
    
    while i < n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        for num_str in input.trim().split_whitespace() {
            if i >= n {
                break;
            }
            let num: i32 = num_str.parse().expect("Invalid input: expected an integer");
            nums.push(num);
            i += 1;
        }
    }
    
    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}