use std::io;

fn is_balanced(num: &str) -> bool {
    let mut even_sum = 0;
    let mut odd_sum = 0;

    // Iterate through each character along with its index
    for (i, c) in num.chars().enumerate() {
        // Convert character to digit, assuming valid input per problem constraints
        let digit = c.to_digit(10).expect("invalid digit in input");
        
        // Accumulate sums based on even/odd index
        if i % 2 == 0 {
            even_sum += digit;
        } else {
            odd_sum += digit;
        }
    }

    even_sum == odd_sum
}

fn main() {
    let mut input = String::new();
    // Read input line from stdin
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Extract first whitespace-separated token, matching C++ cin>> behavior
    let num = input.trim().split_whitespace().next().expect("no input provided");
    
    // Determine and print result in required format
    println!("{}", if is_balanced(num) { "true" } else { "false" });
}