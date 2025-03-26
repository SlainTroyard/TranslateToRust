use std::io;

#[derive(Debug, PartialEq, Eq)]
struct Element {
    value: i32,
    count: i32,
}

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

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements: Vec<Element> = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element { value: j as i32, count: hash[j] });
            }
        }

        elements.sort();

        let mut sum = 0;
        let elements_to_sum = std::cmp::min(elements.len(), x);
        for j in 0..elements_to_sum {
            sum += elements[j].value * elements[j].count;
        }

        answer[i] = sum;
    }

    answer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let k: usize = iter.next().unwrap().parse()?;
    let x: usize = iter.next().unwrap().parse()?;

    let mut nums_size = String::new();
    io::stdin().read_line(&mut nums_size)?;
    let nums_size: usize = nums_size.trim().parse()?;

    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num)?;
        nums.push(num.trim().parse()?);
    }

    let answer = find_x_sum(&nums, k, x);

    for (i, &val) in answer.iter().enumerate() {
        print!("{}", val);
        if i < answer.len() - 1 {
            print!(" ");
        }
    }
    println!();

    Ok(())
}