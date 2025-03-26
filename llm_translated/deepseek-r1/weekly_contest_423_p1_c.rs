fn main() {
    // Read all input tokens as integers
    let input = {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("Failed to read input");
        buffer
    };
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid number"));

    let n = tokens.next().expect("No n provided") as usize;
    let nums: Vec<i32> = {
        let mut temp = Vec::with_capacity(n);
        for _ in 0..n {
            temp.push(tokens.next().expect("Not enough elements in array"));
        }
        temp
    };
    let k = tokens.next().expect("No k provided") as usize;

    // Check for extra input (not present in original C code logic)
    if tokens.next().is_some() {
        panic!("Extra input detected");
    }

    let result = has_increasing_subarrays(&nums, k);
    println!("{}", if result { "true" } else { "false" });
}

fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    if k == 1 {
        return true;
    }

    // Replicate the original double-checking loop (j runs twice)
    for _ in 0..2 {
        // Calculate valid starting index range for first subarray
        let upper = if nums.len() >= 2 * k {
            nums.len() - 2 * k + 1
        } else {
            0
        };

        for i in 0..upper {
            let s = i + k;  // Start index of second subarray
            let mut valid_pairs = 0;

            // Check consecutive elements in both subarrays
            for z in 0..(k - 1) {
                let first_increasing = nums[i + z] < nums[i + z + 1];
                let second_increasing = nums[s + z] < nums[s + z + 1];
                
                if first_increasing && second_increasing {
                    valid_pairs += 1;
                }
            }

            if valid_pairs == k - 1 {
                return true;
            }
        }
    }

    false
}