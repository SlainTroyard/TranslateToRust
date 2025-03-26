use std::io;
use std::vec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    value: i32,
    count: i32,
}

// Function to translate the C findXSum function to Rust
fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = (nums_size as i32 - k + 1) as usize;
    let mut answer: Vec<i32> = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51]; // Using array for hash as in C code, assuming numbers are in range [0, 50]
        for j in 0..k {
            let index = nums[i + (j as usize)];
            if index >= 0 && index <= 50 { // Check bounds as per original C code assumption
                hash[index as usize] += 1;
            }
        }

        let mut elements: Vec<Element> = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element { value: j as i32, count: hash[j] });
            }
        }

        elements.sort_by(|a, b| {
            if b.count == a.count {
                b.value.cmp(&a.value) // Descending value if counts are equal
            } else {
                b.count.cmp(&a.count) // Descending count
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

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut parts = input_line.trim().split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let x: i32 = parts.next().unwrap().parse().unwrap();

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let nums_size: usize = input_line.trim().parse().unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    parts = input_line.trim().split_whitespace();
    for _ in 0..nums_size {
        nums.push(parts.next().unwrap().parse().unwrap());
    }

    let answer = find_x_sum(&nums, k, x);

    for i in 0..answer.len() {
        print!("{} ", answer[i]);
    }
    println!();
}