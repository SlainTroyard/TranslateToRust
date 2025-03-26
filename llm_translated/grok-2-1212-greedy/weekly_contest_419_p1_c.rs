use std::io::{self, BufRead};

// Define a struct to represent an element with value and count
#[derive(Copy, Clone)]
struct Element {
    value: i32,
    count: i32,
}

// Implement comparison for sorting
impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.count == other.count {
            other.value.cmp(&self.value)
        } else {
            other.count.cmp(&self.count)
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Element {}

fn find_x_sum(nums: &[i32], nums_size: usize, k: i32, x: i32) -> Vec<i32> {
    let num_results = nums_size - k as usize + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k as usize {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: hash[j],
                });
            }
        }

        elements.sort();

        let mut sum = 0;
        let elements_to_sum = elements.len().min(x as usize);
        for j in 0..elements_to_sum {
            sum += elements[j].value * elements[j].count;
        }

        answer[i] = sum;
    }

    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    // Read nums_size
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read nums
    let nums: Vec<i32> = lines
        .take(nums_size)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Calculate and print the result
    let answer = find_x_sum(&nums, nums_size, k, x);
    for num in answer {
        print!("{} ", num);
    }
    println!();

    Ok(())
}