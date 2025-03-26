use std::cmp::Ordering;

const TABLE_SIZE: usize = 1031;
const MAX_CHAIN: usize = 32;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Pair {
    val: i32,
    freq: i32,
}

impl Pair {
    fn new(val: i32, freq: i32) -> Self {
        Self { val, freq }
    }
}

fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let k = k as usize;
    let x = x as usize;
    let nums_size = nums.len();
    let result_size = nums_size - k + 1;
    let mut result = Vec::with_capacity(result_size);

    // Initialize hash table
    let mut hash_table = vec![[Pair::default(); MAX_CHAIN]; TABLE_SIZE];

    // Process first window
    for &val in &nums[0..k] {
        let hash = (val as u32 % TABLE_SIZE as u32) as usize;
        let mut found = false;

        // Check if val exists in the bucket
        for entry in &mut hash_table[hash] {
            if entry.freq > 0 && entry.val == val {
                entry.freq += 1;
                found = true;
                break;
            }
        }
        if !found {
            // Find first entry with freq 0
            for entry in &mut hash_table[hash] {
                if entry.freq == 0 {
                    *entry = Pair::new(val, 1);
                    found = true;
                    break;
                }
            }
        }
    }

    // Collect active elements for first window
    let mut active_elements = Vec::with_capacity(k);
    for bucket in hash_table.iter() {
        for &entry in bucket.iter() {
            if entry.freq > 0 {
                active_elements.push(entry);
                if active_elements.len() >= k {
                    break;
                }
            }
        }
        if active_elements.len() >= k {
            break;
        }
    }

    // Sort and calculate sum
    active_elements.sort_unstable_by(|a, b| {
        b.freq.cmp(&a.freq)
            .then(b.val.cmp(&a.val))
    });
    let count = x.min(active_elements.len());
    let sum = active_elements.iter()
        .take(count)
        .map(|p| p.val as i64 * p.freq as i64)
        .sum();
    result.push(sum);

    // Slide the window
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1];
        let out_hash = (out_val as u32 % TABLE_SIZE as u32) as usize;

        // Decrement freq of outgoing value
        for entry in &mut hash_table[out_hash] {
            if entry.val == out_val && entry.freq > 0 {
                entry.freq -= 1;
                break;
            }
        }

        let in_val = nums[i + k - 1];
        let in_hash = (in_val as u32 % TABLE_SIZE as u32) as usize;
        let mut found = false;

        // Check if in_val exists in the bucket
        for entry in &mut hash_table[in_hash] {
            if entry.freq > 0 && entry.val == in_val {
                entry.freq += 1;
                found = true;
                break;
            }
        }
        if !found {
            // Find first entry with freq 0
            for entry in &mut hash_table[in_hash] {
                if entry.freq == 0 {
                    *entry = Pair::new(in_val, 1);
                    found = true;
                    break;
                }
            }
        }

        // Collect active elements again
        active_elements.clear();
        for bucket in hash_table.iter() {
            for &entry in bucket.iter() {
                if entry.freq > 0 {
                    active_elements.push(entry);
                    if active_elements.len() >= k {
                        break;
            }
        }
        }
            if active_elements.len() >= k {
                break;
            }
        }

        // Sort and calculate sum
        active_elements.sort_unstable_by(|a, b| {
            b.freq.cmp(&a.freq)
                .then(b.val.cmp(&a.val))
        });
        let count = x.min(active_elements.len());
        let sum = active_elements.iter()
            .take(count)
            .map(|p| p.val as i64 * p.freq as i64)
            .sum();
        result.push(sum);
    }

    result
}

fn main() {
    // Read input
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let k = tokens.next().unwrap();
    let x = tokens.next().unwrap();
    let nums_size = tokens.next().unwrap();
    let nums: Vec<i32> = tokens.take(nums_size as usize).collect();

    // Compute result
    let result = find_x_sum(&nums, k, x);

    // Print output
    for val in result {
        print!("{} ", val);
    }
    println!();
}