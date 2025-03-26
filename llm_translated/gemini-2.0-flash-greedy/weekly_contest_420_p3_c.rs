fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let mut max_factor;
    let mut j;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max;
            let mut count = 1;
            let original = nums[i];
            loop {
                max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                j = 2;
                while j <= max_factor {
                    if nums[i] % j == 0 {
                        max = max.max(j);
                        if nums[i] / j != j && nums[i] % (nums[i] / j) == 0 {
                            max = max.max(nums[i] / j);
                        }
                    }
                    j += 1;
                }
                if max == i32::MIN || { count *= max; count } == original {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nums_size_str = String::new();
    std::io::stdin().read_line(&mut nums_size_str)?;
    let nums_size: usize = nums_size_str.trim().parse()?;

    let mut nums = Vec::with_capacity(nums_size);
    let mut nums_str = String::new();

    for _ in 0..nums_size {
        nums_str.clear();
        std::io::stdin().read_line(&mut nums_str)?;
        let num: i32 = nums_str.trim().parse()?;
        nums.push(num);
    }

    println!("{}", min_operations(&mut nums));

    Ok(())
}