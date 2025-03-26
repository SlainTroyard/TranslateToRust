use std::collections::HashMap;

pub fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut freq = HashMap::new();

    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut candidates: Vec<_> = freq.keys().cloned().collect();
    candidates.sort_unstable_by(|a, b| b.cmp(a));

    for &n in &candidates {
        let sum_minus_n = total_sum - n;
        if sum_minus_n % 2 != 0 {
            continue;
        }
        let d = sum_minus_n / 2;
        if let Some(&count_d) = freq.get(&d) {
            if d != n || count_d > 1 {
                return n;
            }
        }
    }

    -1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    let n: usize = tokens
        .next()
        .ok_or("No n provided")?
        .parse()
        .map_err(|_| "Invalid n")?;

    let nums: Vec<i32> = tokens
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    if nums.len() != n {
        return Err(format!(
            "Expected {} numbers, got {}",
            n,
            nums.len()
        )
        .into());
    }

    let result = get_largest_outlier(&nums);
    println!("{}", result);
    Ok(())
}