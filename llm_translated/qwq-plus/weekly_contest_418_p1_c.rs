use std::io;

/// Computes the maximum "good" number by optimally permuting the first three elements of the input array.
///
/// The algorithm counts leading zeros in the 7-bit representation of each of the first three numbers.
/// It then determines the best permutation of these three to form the largest possible 21-bit number.
fn max_good_number(nums: &[i32]) -> i32 {
    assert!(nums.len() >= 3);

    // Count leading zeros in the 7-bit representation of the first three numbers
    let (mut count0, mut count1, mut count2) = (0, 0, 0);
    for (i, &num) in nums.iter().take(3).enumerate() {
        let mut count = 0;
        for j in 0..7 {
            let shifted = (num << j) & 64;
            if shifted == 0 {
                count += 1;
            } else {
                break;
            }
        }
        match i {
            0 => count0 = count,
            1 => count1 = count,
            2 => count2 = count,
            _ => unreachable!(),
        }
    }

    let times = [count0, count1, count2];
    let mut store = [0, 1, 2]; // Indices of the three elements to permute

    // Bubble sort-like passes to find the optimal permutation
    for _ in 0..2 {
        for j in 0..2 {
            let a = store[j];
            let b = store[j + 1];

            // Compute values for current and swapped order
            let value1 = (nums[a] << (7 - times[b])) + nums[b];
            let value2 = (nums[b] << (7 - times[a])) + nums[a];

            if value2 >= value1 {
                // Swap to choose the better or equal value
                store.swap(j, j + 1); // Using swap method for clarity
            }
        }
    }

    // Calculate the maximum value using the best permutation
    let a = store[0];
    let b = store[1];
    let c = store[2];
    let total = (nums[a] << (14 - times[b] - times[c])) 
        + (nums[b] << (7 - times[c])) 
        + nums[c];

    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>()
             .expect("Invalid integer in input"));
    
    let num_size = iter.next().expect("Missing number count");
    let nums: Vec<i32> = iter.take(num_size as usize).collect();
    
    assert_eq!(nums.len(), num_size as usize, "Insufficient numbers provided");
    
    println!("{}", max_good_number(&nums));
}