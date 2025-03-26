use std::io;

struct Solution;

impl Solution {
    fn convert_date_to_binary(date: &str) -> String {
        let bin = |x: u32| -> String {
            // Convert to binary with 32 bits
            let binary = format!("{:032b}", x);
            // Find position of first '1' to remove leading zeros
            match binary.find('1') {
                Some(pos) => binary[pos..].to_string(),
                None => "0".to_string() // Handle the case where x is 0
            }
        };
        
        // Extract year, month, and day from the date string
        let year = date[0..4].parse::<u32>().unwrap();
        let month = date[5..7].parse::<u32>().unwrap();
        let day = date[8..10].parse::<u32>().unwrap();
        
        // Format the result with the same pattern as the original
        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}

fn main() {
    // Read date from stdin
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = date.trim().to_string();
    
    // Process and output the result
    let sol = Solution;
    println!("{}", sol.convert_date_to_binary(&date));
}