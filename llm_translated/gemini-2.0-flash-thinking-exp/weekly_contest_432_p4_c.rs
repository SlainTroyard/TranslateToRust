use std::collections::VecDeque;
use std::io;
use std::str::SplitWhitespace;

// Helper function to read a line from stdin and split it into whitespace-separated strings
fn read_line_and_split() -> Result<SplitWhitespace<'static>, io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().split_whitespace())
}

// Helper function to parse the next value from the split iterator
fn parse_next<T>(splits: &mut SplitWhitespace<'static>) -> Result<T, String>
where
    T: std::str::FromStr,
{
    splits.next().ok_or_else(|| "Not enough input values".to_string())?.parse::<T>().map_err(|_| "Failed to parse value".to_string())
}

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<i32>> = vec![Vec::new(); n]; // Adjacency list to store indices 'i' where nums[stackTop(st)] <= nums[i]
    let mut pos_r: Vec<i32> = vec![n as i32; n]; // Array to store the rightmost index 'r' for each 'i' such that nums[i] <= nums[r] for all j in (i, r]
    let mut st: Vec<usize> = Vec::new(); // Stack to find next greater or equal elements

    // Find the next greater or equal element to the right for each element
    for i in 0..n {
        let x = nums[i];
        while !st.is_empty() && x >= nums[*st.last().unwrap()] {
            pos_r[*st.last().unwrap()] = i as i32;
            st.pop();
        }
        if !st.is_empty() {
            g[*st.last().unwrap()].push(i as i32);
        }
        st.push(i);
    }

    let mut ans: i64 = 0;
    let mut cnt: i32 = 0;
    let mut l: usize = 0;
    let mut q: VecDeque<usize> = VecDeque::new(); // Deque to maintain indices of potential maximums in the window

    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[*q.back().unwrap()] <= x {
            q.pop_back();
        }
        q.push_back(r);
        cnt += nums[*q.front().unwrap()] - x; // Add the difference between the current max and current element to cnt

        while cnt > k {
            let out = nums[l];
            for &j_idx in &g[l] {
                let i = j_idx as usize;
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < (r + 1) as i32 { pos_r[i] } else { (r + 1) as i32 };
                cnt -= (out - nums[i]) * (min_pos - (i as i32)); // Reduce cnt based on subarrays starting from l
            }
            l += 1;
            if !q.is_empty() && *q.front().unwrap() < l {
                q.pop_front(); // Remove indices from front that are out of the current window
            }
        }
        ans += (r - l + 1) as i64; // Add the number of valid subarrays ending at r
    }

    ans
}

fn main() -> Result<(), String> {
    let mut splits = read_line_and_split().map_err(|e| e.to_string())?;
    let nums_size: usize = parse_next(&mut splits)?;
    let k: i32 = parse_next(&mut splits)?;

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    let mut splits_nums = read_line_and_split().map_err(|e| e.to_string())?;
    for _ in 0..nums_size {
        nums.push(parse_next(&mut splits_nums)?);
    }

    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);

    Ok(())
}