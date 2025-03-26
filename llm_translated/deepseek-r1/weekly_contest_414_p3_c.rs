// Function to calculate the maximum score based on the given algorithm
pub fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut l = 0;
    let mut r = 1;
    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    ans + ((r - l - 1) as i64) * nums[l] as i64
}

fn main() {
    // Read entire input and split into tokens
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse the first token as the size of the array
    let nums_size = tokens
        .next()
        .expect("No input provided for array size")
        .parse::<usize>()
        .expect("Invalid array size");

    // Parse the next 'nums_size' tokens as i32 to form the array
    let nums: Vec<i32> = tokens
        .by_ref()
        .take(nums_size)
        .map(|s| s.parse().expect("Invalid integer in array"))
        .collect();

    // Ensure the correct number of elements were parsed
    if nums.len() != nums_size {
        panic!("Insufficient elements in input");
    }

    // Calculate and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);
}