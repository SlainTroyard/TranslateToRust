use std::collections::HashMap;

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut map: HashMap<u32, i32> = HashMap::new();
    let mut ans: i64 = 0;

    // First loop to populate the hash map
    for i in 4..(nums.len() - 2) {
        let c = nums[i];
        for j in (i + 2)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let d_div_g = (d / g) as u32;
            let c_div_g = (c / g) as u32;
            let key = (d_div_g << 16) | c_div_g;
            insert_key(&mut map, key, 1);
        }
    }

    // Second loop to compute the answer and adjust the hash map
    for i in 2..(nums.len() - 4) {
        let b = nums[i];
        // First part: count valid pairs (a, b)
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let a_div_g = (a / g) as u32;
            let b_div_g = (b / g) as u32;
            let key = (a_div_g << 16) | b_div_g;
            ans += get_count(&map, key) as i64;
        }

        // Second part: decrement counts for pairs (c, d)
        let c = nums[i + 2];
        for j in (i + 4)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let d_div_g = (d / g) as u32;
            let c_div_g = (c / g) as u32;
            let key = (d_div_g << 16) | c_div_g;
            insert_key(&mut map, key, -1);
        }
    }

    ans
}

// Helper functions to manage the hash map
fn insert_key(map: &mut HashMap<u32, i32>, key: u32, delta: i32) {
    *map.entry(key).or_insert(0) += delta;
}

fn get_count(map: &HashMap<u32, i32>, key: u32) -> i32 {
    *map.get(&key).unwrap_or(&0)
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = iter
        .take(n)
        .map(|x| x.parse().unwrap())
        .collect();

    let result = number_of_subsequences(&nums);
    println!("{}", result);
}