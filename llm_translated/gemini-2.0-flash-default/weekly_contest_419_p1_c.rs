use std::io;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = nums_size - k as usize + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k as usize {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements: Vec<Element> = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: hash[j],
                });
            }
        }

        elements.sort_by(|a, b| {
            if a.count == b.count {
                b.value.cmp(&a.value)
            } else {
                b.count.cmp(&a.count)
            }
        });

        let mut sum = 0;
        let elements_to_sum = std::cmp::min(elements.len() as i32, x) as usize;
        for j in 0..elements_to_sum {
            sum += elements[j].value * elements[j].count;
        }

        answer.push(sum);
    }

    answer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_whitespace();
    let k: i32 = first_line.next().unwrap().parse()?;
    let x: i32 = first_line.next().unwrap().parse()?;

    let nums_size: usize = lines.next().unwrap().parse()?;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let answer = find_x_sum(&nums, k, x);

    for val in answer {
        print!("{} ", val);
    }
    println!();

    Ok(())
}