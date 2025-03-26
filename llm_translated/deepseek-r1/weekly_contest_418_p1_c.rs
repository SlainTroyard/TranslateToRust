use std::io::{self, Read};

fn max_good_number(nums: &[i32]) -> i32 {
    let (mut count0, mut count1, mut count2) = (0, 0, 0);
    
    // Calculate the maximum consecutive leading zeros in the 7th bit for the first three numbers
    for (i, &num) in nums.iter().enumerate().take(3) {
        let count = match i {
            0 => &mut count0,
            1 => &mut count1,
            2 => &mut count2,
            _ => unreachable!(),
        };
        for j in 0..7 {
            let shifted = num.wrapping_shl(j as u32);
            let s = shifted & 0x40; // 0x40 is the 7th bit (64 in decimal)
            if s == 0 {
                *count += 1;
            } else {
                break;
            }
        }
    }
    
    let times = [count0, count1, count2];
    let mut store = [0, 1, 2];
    
    // Custom sorting based on combination values
    for _ in 1..=2 {
        for j in 0..=1 {
            let a = store[j];
            let b = store[j + 1];
            let value1 = (nums[a].wrapping_shl((7 - times[b]) as u32)).wrapping_add(nums[b]);
            let value2 = (nums[b].wrapping_shl((7 - times[a]) as u32)).wrapping_add(nums[a]);
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }
    
    // Calculate the final combined value
    let a = store[0];
    let b = store[1];
    let c = store[2];
    let part1 = nums[a].wrapping_shl((14 - times[b] - times[c]) as u32);
    let part2 = nums[b].wrapping_shl((7 - times[c]) as u32);
    let part3 = nums[c];
    
    part1.wrapping_add(part2).wrapping_add(part3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    let num_size: usize = tokens.next().expect("Missing numSize").parse().expect("Invalid numSize");
    let nums: Vec<i32> = tokens.take(num_size)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    println!("{}", max_good_number(&nums));
}