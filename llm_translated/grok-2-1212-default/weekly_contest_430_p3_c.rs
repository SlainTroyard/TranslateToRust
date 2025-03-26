use std::io::{self, BufRead};
use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    fn hash_insert(hash_map: &mut HashMap<i32, i32>, key: i32, value: i32) {
        *hash_map.entry(key).or_insert(0) += value;
    }

    fn hash_get(hash_map: &HashMap<i32, i32>, key: i32) -> i32 {
        *hash_map.get(&key).unwrap_or(&0)
    }

    let mut ans = 0;

    for i in 4..nums.len() - 2 {
        let c = nums[i];
        for j in i + 2..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = (d / g) << 16 | (c / g);
            hash_insert(&mut hash_map, key, 1);
        }
    }

    for i in 2..nums.len() - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = (a / g) << 16 | (b / g);
            ans += hash_get(&hash_map, key) as i64;
        }

        let c = nums[i + 2];
        for j in i + 4..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = (d / g) << 16 | (c / g);
            hash_insert(&mut hash_map, key, -1);
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = number_of_subsequences(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}