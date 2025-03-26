use std::io::{self, Write};

fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;

    for (i, &num) in nums.iter().enumerate() {
        for j in 0..7 {
            let s = (num << j) & 64;
            if s == 0 {
                match i {
                    0 => nums1 += 1,
                    1 => num2 += 1,
                    2 => num3 += 1,
                    _ => (),
                }
            } else {
                break;
            }
        }
    }

    let mut times = vec![nums1, num2, num3];
    let mut store = vec![0, 1, 2];

    for i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j]] << (7 - times[store[j + 1]])) + nums[store[j + 1]];
            let value2 = (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }

    (nums[store[0]] << (14 - times[store[1]] - times[store[2]]))
        + (nums[store[1]] << (7 - times[store[2]]))
        + nums[store[2]]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_size: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let result = max_good_number(&nums);
    println!("{}", result);
}