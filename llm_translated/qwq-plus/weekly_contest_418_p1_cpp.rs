use std::io;

mod solution {
    pub fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        nums.sort_by(|a, b| {
            let len_a = compute_length(*a);
            let len_b = compute_length(*b);
            let a_u = *a as u32;
            let b_u = *b as u32;
            let ab = (a_u << len_b) | b_u;
            let ba = (b_u << len_a) | a_u;
            if ab > ba {
                std::cmp::Ordering::Less
            } else if ab < ba {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mut ans = 0;
        for &x in nums.iter() {
            let len_x = compute_length(x);
            ans = (ans << len_x) | x;
        }
        ans
    }

    fn compute_length(x: i32) -> u32 {
        if x == 0 {
            1
        } else {
            (32 - (x as u32).leading_zeros()) as u32
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return Err("No input provided".into());
    }
    let n: usize = tokens[0].parse()?;
    let expected_tokens = 1 + n;
    if tokens.len() < expected_tokens {
        return Err("Not enough numbers provided".into());
    }
    let nums: Vec<i32> = tokens[1..expected_tokens]
        .iter()
        .map(|&s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut nums = nums;
    let ans = solution::max_good_number(&mut nums);
    println!("{}", ans);
    Ok(())
}