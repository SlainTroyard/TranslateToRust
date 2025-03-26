use std::io;

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let bin = |x: u32| -> String {
            // Convert number to binary string and remove leading zeros
            format!("{:b}", x)
        };
        
        // Parse the date components
        let year = date[0..4].parse::<u32>().unwrap();
        let month = date[5..7].parse::<u32>().unwrap();
        let day = date[8..10].parse::<u32>().unwrap();
        
        // Format the result in the same format as the original date
        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}

fn main() {
    // Read input date string
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    date = date.trim().to_string(); // Remove trailing newline
    
    // Create solution and call the method
    let sol = Solution;
    println!("{}", sol.convert_date_to_binary(date));
}