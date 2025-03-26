fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2];
    let n = nums.len();
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] == nums[j] {
                if count < 2 {
                    result[count] = nums[i];
                    count += 1;
                }
                break;
            }
        }
        if count == 2 {
            break;
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let initial_num_size = tokens[0];
    let num_size = initial_num_size + 2;

    let nums = &tokens[1..1 + num_size as usize];
    let result = get_sneaky_numbers(nums);

    for num in result {
        print!("{} ", num);
    }
    println!();
}