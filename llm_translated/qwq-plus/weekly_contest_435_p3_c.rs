use std::io;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn minimum_increments(
    nums: &[i32],
    target: &[i32],
) -> Result<i32, Box<dyn std::error::Error>> {
    let m = target.len();
    let n = nums.len();

    // Compute g array
    let mut g = vec![1; 1 << m];
    for i in 0..(1 << m) {
        let mut current = 1;
        for j in 0..m {
            if (i & (1 << j)) != 0 {
                let t = target[j] as i64;
                let g_val = gcd(current, t);
                current = (current / g_val) * t;
            }
        }
        g[i] = current;
    }

    // Dynamic programming setup
    let inf = 1e18 as i64;
    let mut f_prev = vec![inf; 1 << m];
    f_prev[0] = 0;

    for i in 1..=n {
        let current_num = nums[i - 1] as i64;
        let mut f_current = f_prev.clone();

        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let prev_mask = j ^ k;
                let v = ((current_num + g[k] - 1) / g[k]) * g[k] - current_num;
                if f_prev[prev_mask] + v < f_current[j] {
                    f_current[j] = f_prev[prev_mask] + v;
                }
                k = (k - 1) & j;
            }
        }

        f_prev = f_current;
    }

    let result = f_prev[(1 << m) - 1];
    if result >= inf {
        Ok(-1)
    } else {
        Ok(result as i32)
    }
}

fn read_all_numbers() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let numbers: Result<Vec<_>, _> = tokens.iter().map(|s| s.parse::<i32>()).collect();
    numbers.map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = read_all_numbers()?;
    let mut iter = numbers.iter();
    let n = *iter.next().ok_or("Not enough numbers for n")?;
    let m = *iter.next().ok_or("Not enough numbers for m")?;

    let nums: Vec<i32> = iter.take(n as usize).copied().collect();
    let target: Vec<i32> = iter.take(m as usize).copied().collect();

    if nums.len() != n as usize {
        return Err("Not enough numbers for nums".into());
    }
    if target.len() != m as usize {
        return Err("Not enough numbers for target".into());
    }

    let result = minimum_increments(&nums, &target)?;
    println!("{}", result);
    Ok(())
}