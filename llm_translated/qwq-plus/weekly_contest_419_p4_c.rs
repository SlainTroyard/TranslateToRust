use std::io;

const TABLE_SIZE: usize = 1031;
const MAX_CHAIN: usize = 32;

#[derive(Clone, Copy)]
struct Pair {
    val: i32,
    freq: i32,
}

pub fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let n = nums.len();
    let mut result = Vec::with_capacity(n - k + 1);
    let mut hash_table: Vec<[Pair; MAX_CHAIN]> = vec![
        [Pair { val: 0, freq: 0 }; MAX_CHAIN];
        TABLE_SIZE
    ];

    let mut active_elements = Vec::with_capacity(k);

    // Initialize first window
    for &val in nums.iter().take(k) {
        let hash = (val as u32 % (TABLE_SIZE as u32)) as usize;
        let mut found = false;

        for j in 0..MAX_CHAIN {
            let pair = &mut hash_table[hash][j];
            if pair.freq == 0 {
                break;
            }
            if pair.val == val {
                pair.freq += 1;
                found = true;
                break;
            }
        }

        if !found {
            for j in 0..MAX_CHAIN {
                let pair = &mut hash_table[hash][j];
                if pair.freq == 0 {
                    pair.val = val;
                    pair.freq = 1;
                    break;
                }
            }
        }
    }

    // Collect active elements for first window
    active_elements.clear();
    for bucket in hash_table.iter() {
        for pair in bucket.iter() {
            if pair.freq > 0 {
                active_elements.push(*pair);
                if active_elements.len() >= k {
                    break;
                }
            }
        }
        if active_elements.len() >= k {
            break;
        }
    }

    // Sort and compute sum for first window
    active_elements.sort_by(|a, b| {
        b.freq.cmp(&a.freq).then(b.val.cmp(&a.val))
    });
    let count = std::cmp::min(x, active_elements.len());
    let mut sum: i64 = 0;
    for i in 0..count {
        sum += (active_elements[i].val as i64) * (active_elements[i].freq as i64);
    }
    result.push(sum);

    // Process remaining windows
    for i in 1..=(n - k) {
        // Remove outgoing element (nums[i-1])
        let out_val = nums[i - 1];
        let hash_out = (out_val as u32 % (TABLE_SIZE as u32)) as usize;
        let mut found = false;
        for j in 0..MAX_CHAIN {
            let pair = &mut hash_table[hash_out][j];
            if pair.freq == 0 {
                break;
            }
            if pair.val == out_val {
                pair.freq -= 1;
                found = true;
                break;
            }
        }

        // Add incoming element (nums[i + k -1])
        let in_val = nums[i + k - 1];
        let hash_in = (in_val as u32 % (TABLE_SIZE as u32)) as usize;
        let mut found = false;
        for j in 0..MAX_CHAIN {
            let pair = &mut hash_table[hash_in][j];
            if pair.freq == 0 {
                break;
            }
            if pair.val == in_val {
                pair.freq += 1;
                found = true;
                break;
            }
        }

        if !found {
            for j in 0..MAX_CHAIN {
                let pair = &mut hash_table[hash_in][j];
                if pair.freq == 0 {
                    pair.val = in_val;
                    pair.freq = 1;
                    break;
                }
            }
        }

        // Re-collect active elements
        active_elements.clear();
        for bucket in hash_table.iter() {
            for pair in bucket.iter() {
                if pair.freq > 0 {
                    active_elements.push(*pair);
                    if active_elements.len() >= k {
                        break;
                    }
                }
            }
            if active_elements.len() >= k {
                break;
            }
        }

        // Sort and compute sum
        active_elements.sort_by(|a, b| {
            b.freq.cmp(&a.freq).then(b.val.cmp(&a.val))
        });
        let count = std::cmp::min(x, active_elements.len());
        let mut sum = 0;
        for i in 0..count {
            sum += (active_elements[i].val as i64) * (active_elements[i].freq as i64);
        }
        result.push(sum);
    }

    result
}

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).expect("Failed to read input");
    let first_line: Vec<&str> = buffer.trim().split_whitespace().collect();
    let k = first_line[0].parse::<usize>().unwrap();
    let x = first_line[1].parse::<usize>().unwrap();

    buffer.clear();
    stdin.read_line(&mut buffer).expect("Failed to read numsSize");
    let nums_size = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    stdin.read_line(&mut buffer).expect("Failed to read nums array");
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), nums_size);

    let result = find_x_sum(&nums, k, x);

    for num in result.iter() {
        print!("{} ", num);
    }
    println!();
}