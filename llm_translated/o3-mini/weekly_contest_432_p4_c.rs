use std::collections::VecDeque;
use std::io::{self, Write};

/// Counts the number of non-decreasing subarrays with a sum condition.
/// This function preserves the algorithm and logic from the original C code.
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();

    // Create a graph "g" as a vector of vectors.
    // Each g[i] will store indices in a preallocated vector with capacity 10.
    let mut g: Vec<Vec<usize>> = Vec::with_capacity(n);
    for _ in 0..n {
        g.push(Vec::with_capacity(10));
    }

    // Initialize pos_r with default value n.
    let mut pos_r = vec![n; n];

    // Use a stack (implemented with Vec) to determine the next greater or equal element.
    let mut st: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        let x = nums[i];
        // While the current element is greater or equal than the element at the top index of stack...
        while let Some(&top_idx) = st.last() {
            if x >= nums[top_idx] {
                // Set pos_r for the index at the top of the stack.
                pos_r[top_idx] = i;
                st.pop();
            } else {
                break;
            }
        }
        // If the stack is not empty, add the current index to g of the top of the stack.
        if let Some(&top_idx) = st.last() {
            g[top_idx].push(i);
        }
        // Push the current index onto the stack.
        st.push(i);
    }

    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    let mut l: usize = 0;

    // Use a deque (VecDeque) to keep track of indices whilst sliding the window.
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(n);

    // Iterate over each possible right endpoint `r` of the subarray.
    for r in 0..n {
        let x = nums[r];
        // Remove elements from the back while they are <= current x.
        while let Some(&back) = deque.back() {
            if nums[back] <= x {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(r);
        // Update cnt using the current minimum value in the window.
        cnt += (nums[*deque.front().unwrap()] - x) as i64;

        // If the running count exceeds the threshold k,
        // move the left boundary to shrink the window and adjust cnt.
        while cnt > k as i64 {
            let out = nums[l];
            // For all indices stored in g[l], update cnt.
            for &j in &g[l] {
                if j > r {
                    break;
                }
                // Use the minimum of pos_r[j] and (r + 1) as in C code.
                let min_pos = if pos_r[j] < r + 1 { pos_r[j] } else { r + 1 };
                cnt -= ((out - nums[j]) as i64) * ((min_pos - j) as i64);
            }
            l += 1;
            // Remove the leftmost element from deque if it is now out of the window.
            if let Some(&front) = deque.front() {
                if front < l {
                    deque.pop_front();
                }
            }
        }
        ans += (r - l + 1) as i64;
    }
    ans
}

fn main() {
    // Read the entire input.
    let input = {
        let mut s = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut s) {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
        s
    };

    // Split input by whitespace to parse tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        let _ = writeln!(io::stderr(), "Error reading input for numsSize and k");
        std::process::exit(1);
    }

    // Parse nums_size and k from the first two tokens.
    let nums_size: usize = match tokens[0].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for numsSize");
            std::process::exit(1);
        }
    };
    let k: i32 = match tokens[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for k");
            std::process::exit(1);
        }
    };

    // Ensure that there are enough tokens for the nums array.
    if tokens.len() < 2 + nums_size {
        eprintln!("Error reading input for nums array");
        std::process::exit(1);
    }

    // Parse the nums array.
    let mut nums = Vec::with_capacity(nums_size);
    for i in 0..nums_size {
        let token = tokens[2 + i];
        let num = match token.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error reading input for nums[{}]", i);
                std::process::exit(1);
            }
        };
        nums.push(num);
    }

    // Compute the result using the translated algorithm.
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output the result.
    println!("{}", result);
}