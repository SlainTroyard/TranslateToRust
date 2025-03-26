use std::io;

fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let mut max_factor = 0;
    let mut j = 0;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max = i32::MIN;
            let mut count = 1;
            let original = nums[i];
            loop {
                max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max = max.max(j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = max.max(nums[i] / j);
                        }
                    }
                }
                if max == i32::MIN || (count *= max) == original {
                    return -1;
                } else {
                    nums[i] /= max;
                    if nums[i] <= nums[i + 1] {
                        res += 1;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Please type a number!");
    
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums.push(input.trim().parse().expect("Please type a number!"));
    }

    let result = min_operations(&mut nums);
    println!("{}", result);
}