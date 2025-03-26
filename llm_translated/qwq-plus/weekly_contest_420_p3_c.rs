use std::io;

/// Computes the minimum number of operations to make the array non-decreasing by dividing elements.
///
/// # Arguments
/// * `nums` - A mutable slice of integers to process.
///
/// # Returns
/// The minimum operations required, or -1 if impossible.
pub fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let n = nums.len();
    if n == 1 {
        return 0;
    }
    for i in (0..n - 1).rev() {
        if nums[i] > nums[i + 1] {
            let original = nums[i];
            let mut count = 1;
            loop {
                let mut max = i32::MIN;
                let current = nums[i];
                let max_factor = (current as f64).sqrt() as i32 + 1;
                for j in 2..=max_factor {
                    if current % j == 0 {
                        let div = current / j;
                        let candidate = j.max(div);
                        if candidate > max {
                            max = candidate;
                        }
                    }
                }
                if max == i32::MIN {
                    return -1;
                }
                let new_count = count * max;
                if new_count == original {
                    return -1;
                }
                count = new_count;
                nums[i] /= max;
                if nums[i] <= nums[i + 1] {
                    res += 1;
                    break;
                }
            }
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line as the size of the array
    let n_line = lines.next().expect("No input").unwrap();
    let n: usize = n_line.trim().parse().expect("Invalid numsSize");

    // Read the next lines as the elements of the array
    let nums: Vec<i32> = lines
        .map(|line| line.unwrap().trim().parse().unwrap())
        .take(n)
        .collect();

    // Convert to mutable vector to allow in-place modifications
    let mut nums = nums;
    let result = min_operations(&mut nums);
    println!("{}", result);
}