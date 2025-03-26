use std::io;

pub fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;
    let mut f1 = [0; 51];
    let mut max_f1 = 0;
    let mut f2 = 0;

    for &x in nums.iter() {
        // Update f2: max of previous f2 and max_f1, plus 1 if x == k
        let new_f2 = f2.max(max_f1) + if x == k { 1 } else { 0 };
        f2 = new_f2;

        // Update f1[x]: max of previous f1[x] and f0, then add 1
        let current_f1_x = f1[x as usize];
        f1[x as usize] = current_f1_x.max(f0) + 1;

        // Update f0: count of k's encountered so far
        f0 += if x == k { 1 } else { 0 };

        // Update max_f1 with the current f1[x]
        if f1[x as usize] > max_f1 {
            max_f1 = f1[x as usize];
        }
    }

    // Return the maximum of max_f1 and f2
    max_f1.max(f2)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line for n and k
    let first_line = lines.next().expect("No input line").unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().expect("n not found").parse().expect("Invalid n");
    let k: i32 = parts.next().expect("k not found").parse().expect("Invalid k");

    // Collect all remaining numbers into a vector
    let mut nums = Vec::new();
    for line in lines {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            nums.push(token.parse().expect("Invalid number"));
        }
    }

    // Ensure exactly n elements (as per problem constraints)
    let nums_slice = &nums[..n];

    // Compute result and print
    let result = max_frequency(nums_slice, k);
    println!("{}", result);
}