struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0i32; 51];
        for j in 0..k {
            let num = nums[i + j] as usize;
            hash[num] += 1;
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

        elements.sort_by(|a, b| {
            if a.count == b.count {
                // Descending order of value when counts are equal
                b.value.cmp(&a.value)
            } else {
                // Descending order of count
                b.count.cmp(&a.count)
            }
        });

        let elements_to_sum = std::cmp::min(x, elements.len());
        let sum: i32 = elements.iter()
            .take(elements_to_sum)
            .map(|e| e.value * e.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let first_line = lines[0];
    let k_x: Vec<&str> = first_line.split_whitespace().collect();
    let k: usize = k_x[0].parse().unwrap();
    let x: usize = k_x[1].parse().unwrap();

    let nums_size: usize = lines[1].parse().unwrap();

    let nums_line = lines[2];
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let answer = find_x_sum(&nums, k, x);

    // Output with spaces between elements and a trailing newline
    for num in &answer {
        print!("{} ", num);
    }
    println!();
}