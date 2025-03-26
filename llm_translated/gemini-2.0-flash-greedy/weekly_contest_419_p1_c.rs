use std::io;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Element {
    value: i32,
    count: i32,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count == other.count {
            other.value.cmp(&self.value) // Sort by value descending if counts are equal
        } else {
            other.count.cmp(&self.count) // Sort by count descending
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

        elements.sort(); // Use Rust's sort which requires Ord trait

        let mut sum = 0;
        let elements_to_sum = std::cmp::min(elements.len() as i32, x);
        for j in 0..elements_to_sum as usize {
            sum += elements[j].value * elements[j].count;
        }

        answer.push(sum);
    }

    answer
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
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