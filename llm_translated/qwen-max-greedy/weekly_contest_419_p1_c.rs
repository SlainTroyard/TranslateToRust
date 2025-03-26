use std::io::{self, BufRead, Write};

struct Element {
    value: i32,
    count: i32,
}

fn cmp(a: &Element, b: &Element) -> std::cmp::Ordering {
    if a.count == b.count {
        b.value.cmp(&a.value)
    } else {
        b.count.cmp(&a.count)
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements: Vec<Element> = Vec::new();
        for (j, &count) in hash.iter().enumerate() {
            if count > 0 {
                elements.push(Element { value: j as i32, count });
            }
        }

        elements.sort_by(cmp);

        let mut sum = 0;
        let elements_to_sum = elements.len().min(x);
        for element in elements.iter().take(elements_to_sum) {
            sum += element.value * element.count;
        }

        answer[i] = sum;
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read k and x
    stdin.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    // Read numsSize
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let nums_size: usize = buffer.trim().parse().unwrap();

    // Read nums
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Calculate the result
    let answer = find_x_sum(&nums, k, x);

    // Print the result
    for (i, &sum) in answer.iter().enumerate() {
        write!(stdout, "{}", sum).unwrap();
        if i < answer.len() - 1 {
            write!(stdout, " ").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}