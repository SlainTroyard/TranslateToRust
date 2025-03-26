use std::io::{self, BufRead};

/// Calculate the greatest common divisor (GCD) of two numbers.
fn gcd(a: i64, b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

/// Calculate the least common multiple (LCM) of two numbers.
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

/// Calculate the maximum score based on the given algorithm.
fn max_score(nums: &Vec<i64>) -> i64 {
    let nums_size = nums.len();
    if nums_size == 1 {
        return nums[0] * nums[0];
    }

    let mut lcms = vec![0; nums_size];
    let mut gcds = vec![0; nums_size];
    lcms[nums_size - 1] = gcds[nums_size - 1] = nums[nums_size - 1];

    for i in (0..nums_size - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    let mut prelcm = nums[0];
    let mut pregcd = nums[0];

    for i in 1..nums_size - 1 {
        ans = ans.max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]));
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }

    ans.max(prelcm * pregcd)
}

fn main() {
    // Read the number of elements from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut nums_size_str = String::new();
    stdin_lock.read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Please type a number!");

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_str = String::new();
        stdin_lock.read_line(&mut num_str).expect("Failed to read line");
        let num: i64 = num_str.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate and print the result
    let result = max_score(&nums);
    println!("{}", result);
}