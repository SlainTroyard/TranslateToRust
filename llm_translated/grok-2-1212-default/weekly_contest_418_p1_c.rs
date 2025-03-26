use std::io::{self, BufRead};

fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;
    for i in 0..nums.len() {
        for j in 0..7 {
            if i == 0 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    nums1 += 1;
                } else {
                    break;
                }
            } else if i == 1 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num2 += 1;
                } else {
                    break;
                }
            } else if i == 2 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num3 += 1;
                } else {
                    break;
                }
            }
        }
    }
    let mut times = vec![0; 3];
    times[0] = nums1;
    times[1] = num2;
    times[2] = num3;
    let mut store = vec![0; 3];
    store[1] = 1;
    store[2] = 2;
    for i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j]] << (7 - times[store[j + 1]])) +
                         nums[store[j + 1]];
            let value2 =
                (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }
    (nums[store[0]] << (14 - times[store[1]] - times[store[2]])) +
    (nums[store[1]] << (7 - times[store[2]])) + nums[store[2]]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let num_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap()?.trim().parse().unwrap();
        nums.push(num);
    }

    // Calculate and print the result
    let result = max_good_number(&nums);
    println!("{}", result);

    Ok(())
}