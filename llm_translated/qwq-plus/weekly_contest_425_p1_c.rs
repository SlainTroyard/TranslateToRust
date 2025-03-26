use std::io;

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let mut min_sum: Option<i32> = None;
    for current_length in l..=r {
        let mut current_sum = 0;
        let mut current_min: Option<i32> = None;
        for (i, &num) in nums.iter().enumerate() {
            current_sum += num;
            if i >= current_length {
                current_sum -= nums[i - current_length];
            }
            if i >= current_length.saturating_sub(1) {
                if current_sum > 0 {
                    if let Some(c_min) = current_min {
                        if current_sum < c_min {
                            current_min = Some(current_sum);
                        }
                    } else {
                        current_min = Some(current_sum);
                    }
                }
            }
        }
        if let Some(c_min) = current_min {
            min_sum = match min_sum {
                Some(m) => Some(m.min(c_min)),
                None => Some(c_min),
            };
        }
    }
    min_sum.unwrap_or(-1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input.");

    let mut iter = input.split_whitespace();

    let nums_size = iter.next()
        .expect("Missing numsSize.")
        .parse::<usize>()
        .expect("Invalid numsSize.");

    let nums: Vec<i32> = iter.by_ref()
        .take(nums_size)
        .map(|s| s.parse().expect("Invalid number in array."))
        .collect();

    if nums.len() != nums_size {
        eprintln!("Insufficient elements for array.");
        std::process::exit(1);
    }

    let l = iter.next()
        .expect("Missing l.")
        .parse::<usize>()
        .expect("Invalid l.");

    let r = iter.next()
        .expect("Missing r.")
        .parse::<usize>()
        .expect("Invalid r.");

    let result = minimum_sum_subarray(&nums, l, r);
    println!("{}", result);
}