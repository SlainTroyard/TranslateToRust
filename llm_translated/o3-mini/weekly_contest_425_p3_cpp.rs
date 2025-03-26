use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, Read};

/// This function implements the same logic as the provided C++ solution.
/// It manipulates the `nums` slice in multiple phases to minimize the sum.
fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
    let n = nums.len();
    // Phase 0: Sort the array in ascending order.
    nums.sort();

    // Find the boundaries for the three sections using the lower_bound idea.
    // m1: first index where nums[idx] >= k
    // m2: first index where nums[idx] >= (2*k - 1)
    let m1 = nums.iter().position(|&x| x >= k).unwrap_or(n);
    let m2 = nums.iter().position(|&x| x >= 2 * k - 1).unwrap_or(n);

    // Use a HashSet to track candidate indices for swapping in phase 4.
    let mut candidates = HashSet::new();
    let mut swap_cnt = 0;

    // Phase 1:
    // Process the largest numbers (from the end of the array to index m2) and apply op1 then op2.
    let mut i = n as isize - 1;
    while i >= m2 as isize && op1 > 0 {
        let idx = i as usize;
        // Apply op1: reduce the number by converting it to (x+1)/2.
        nums[idx] = (nums[idx] + 1) / 2;
        op1 -= 1;
        // If op2 is available, also subtract k.
        if op2 > 0 {
            nums[idx] -= k;
            op2 -= 1;
        }
        i -= 1;
    }

    // Phase 2:
    // Process the middle section from index m1 up to the current i (inclusive as long as op2 remains).
    let mut j = m1;
    // Ensure that j (usize) is compared against i (isize): only proceed if j as isize <= i.
    while (j as isize) <= i && op2 > 0 {
        // If k is odd and nums[j] is even, mark this index as a candidate for later swap.
        if k % 2 == 1 && nums[j] % 2 == 0 {
            candidates.insert(j);
        }
        // Apply op2: subtract k.
        nums[j] -= k;
        op2 -= 1;
        j += 1;
    }

    // Phase 3:
    // Apply op1 to the remaining untouched middle section from the end (index i) down to index j.
    // Continue while there are operations left and valid indices.
    while i >= j as isize && op1 > 0 {
        let idx = i as usize;
        // If k is odd and nums[idx] is odd, count this occurrence for a swap possibility.
        if k % 2 == 1 && nums[idx] % 2 == 1 {
            swap_cnt += 1;
        }
        // Apply op1: update the number.
        nums[idx] = (nums[idx] + 1) / 2;
        op1 -= 1;
        i -= 1;
    }

    // Phase 4:
    // For the remaining untouched numbers in the left section (indices 0 to j-1),
    // collect them into a vector along with their indices.
    let mut arr: Vec<(i32, usize)> = (0..j)
        .map(|idx| (nums[idx], idx))
        .collect();
    // Sort arr in ascending order so that the largest element is at the end.
    arr.sort();

    // Greedily apply the remaining op1 operations on the largest elements.
    while op1 > 0 && !arr.is_empty() {
        if let Some((num, idx)) = arr.pop() {
            // Apply op1 operation to the selected number.
            nums[idx] = (num + 1) / 2;
            op1 -= 1;
            // If this index was marked as a candidate and we have a swap opportunity, apply the swap.
            if candidates.contains(&idx) && swap_cnt > 0 {
                swap_cnt -= 1;
                nums[idx] -= 1;
            }
        }
    }

    // Return the total sum of the modified numbers.
    nums.iter().sum()
}

fn main() -> io::Result<()> {
    // Read the entire input from STDIN.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input into whitespace-separated tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    // Parse the first four tokens: n, k, op1, op2
    if tokens.len() < 4 {
        return Ok(()); // Not enough input, exit gracefully.
    }
    let n: usize = tokens[0].parse().expect("Failed to parse n");
    let k: i32 = tokens[1].parse().expect("Failed to parse k");
    let op1: i32 = tokens[2].parse().expect("Failed to parse op1");
    let op2: i32 = tokens[3].parse().expect("Failed to parse op2");
    
    // Ensure that there are enough numbers for the nums vector.
    if tokens.len() < 4 + n {
        return Ok(()); // Not enough numbers provided.
    }
    let mut nums: Vec<i32> = tokens[4..4 + n]
        .iter()
        .map(|&s| s.parse().expect("Failed to parse a number in nums"))
        .collect();
    
    // Call the solution function and output the result.
    let result = min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);
    
    Ok(())
}