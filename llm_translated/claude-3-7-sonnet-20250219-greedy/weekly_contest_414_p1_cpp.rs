use std::io;

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let bin = |x: u32| -> String {
            // Convert to binary string and remove leading zeros
            format!("{:b}", x)
        };
        
        // Parse the date components
        let year = date[0..4].parse::<u32>().unwrap();
        let month = date[5..7].parse::<u32>().unwrap();
        let day = date[8..10].parse::<u32>().unwrap();
        
        // Format the result
        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}

fn main() {
    // Read input date
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = date.trim().to_string();
    
    // Create solution instance and call the function
    let sol = Solution;
    println!("{}", sol.convert_date_to_binary(date));
}