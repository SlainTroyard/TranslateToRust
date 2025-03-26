use std::cmp::Ordering;

struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let num_results = nums.len() - k + 1;

    for i in 0..num_results {
        let mut frequency = [0i32; 51];
        for j in 0..k {
            let num = nums[i + j];
            frequency[num as usize] += 1;
        }

        let mut elements = Vec::new();
        for j in 0..51 {
            if frequency[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: frequency[j],
                });
            }
        }

        elements.sort_by(|a, b| {
            if a.count > b.count {
                Ordering::Less
            } else if a.count < b.count {
                Ordering::Greater
            } else {
                if a.value > b.value {
                    Ordering::Less
                } else if a.value < b.value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        });

        let sum = elements
            .iter()
            .take(x)
            .map(|e| e.value * e.count)
            .sum::<i32>();
        result.push(sum);
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>());

    let k = tokens.next().unwrap_or(Err("No k"))?;
    let x = tokens.next().unwrap_or(Err("No x"))?;
    let nums_size = tokens.next().unwrap_or(Err("No nums_size"))?;
    let nums: Vec<i32> = tokens
        .take(nums_size as usize)
        .collect::<Result<_, _>>()?;

    let result = find_x_sum(&nums, k as usize, x as usize);

    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}