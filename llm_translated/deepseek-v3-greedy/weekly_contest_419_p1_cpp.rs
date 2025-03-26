use std::collections::HashMap;
use std::io;

fn main() {
    // Read k and x from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let k: usize = iter.next().unwrap().parse().expect("Invalid input for k");
    let x: usize = iter.next().unwrap().parse().expect("Invalid input for x");

    // Read numsSize from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Invalid input for numsSize");

    // Read nums from stdin
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input for nums");
        nums.push(num);
    }

    // Call the function and get the result
    let res = find_x_sum(&nums, k, x);

    // Print the result
    for val in res {
        print!("{} ", val);
    }
    println!();
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let mut mp = HashMap::new();
    let mut res = Vec::new();
    let mut l = 0;

    for r in 0..nums.len() {
        *mp.entry(nums[r]).or_insert(0) += 1;

        if r - l + 1 == k {
            // Convert the HashMap to a Vec of pairs and sort it
            let mut vec: Vec<_> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect();
            vec.sort_by(|&(num1, cnt1), &(num2, cnt2)| {
                if cnt1 == cnt2 {
                    num2.cmp(&num1) // If counts are equal, sort by num in descending order
                } else {
                    cnt2.cmp(&cnt1) // Sort by count in descending order
                }
            });

            // Calculate the sum of the top x elements
            let sum: i32 = vec.iter().take(x).map(|&(num, cnt)| num * cnt).sum();
            res.push(sum);

            // Move the window
            *mp.get_mut(&nums[l]).unwrap() -= 1;
            if mp[&nums[l]] == 0 {
                mp.remove(&nums[l]);
            }
            l += 1;
        }
    }

    res
}