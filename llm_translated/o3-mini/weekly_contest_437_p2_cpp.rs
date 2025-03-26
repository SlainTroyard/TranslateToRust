use std::io::{self, BufRead};

fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    // Sort pizzas in descending order (greater first)
    pizzas.sort_unstable_by(|a, b| b.cmp(a));
    
    // Calculate the number of days (each day uses 4 pizzas)
    let days = pizzas.len() / 4;
    // Calculate odd, which is (days + 1) / 2
    let odd = (days + 1) / 2;
    
    let mut ans: i64 = 0;
    // Sum the first 'odd' pizzas from the sorted vector
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }
    // Then add the pizzas at positions: odd + i*2 + 1 for i in 0..(days / 2)
    for i in 0..(days / 2) {
        let index = odd + i * 2 + 1;
        // Since we know the index is within bounds because days was computed from pizzas.len()
        ans += pizzas[index] as i64;
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Use stdin locked reader for efficient input reading
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line which should contain n, the number of pizzas
    let n_line = lines.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Expected a line for n"))??;
    let n: usize = n_line.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid number for n: {}", e))
    })?;

    // Read numbers from input. We allow multiple numbers per line.
    let mut pizzas = Vec::with_capacity(n);
    while pizzas.len() < n {
        // Get the next line
        let line = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Expected more lines for pizza weights"))??;
        // Split the line by whitespace and parse each number
        for token in line.trim().split_whitespace() {
            // Stop if we already have read n pizzas
            if pizzas.len() >= n {
                break;
            }
            let num: i32 = token.parse().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Invalid pizza weight: {}", e))
            })?;
            pizzas.push(num);
        }
    }

    // Get the answer using the max_weight function
    let answer = max_weight(pizzas);
    // Print the answer to stdout followed by a newline
    println!("{}", answer);

    Ok(())
}