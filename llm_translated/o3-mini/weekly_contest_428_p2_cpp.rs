use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Performs a Bellman-Ford-like relaxation over the provided currency conversion pairs.
/// 
/// For each conversion pair, it relaxes the best known conversion rates by trying both directions.
/// This function iterates as many times as there are conversion pairs.
fn bellman(
    best: &mut HashMap<String, f64>,
    pairs: &Vec<Vec<String>>,
    rates: &Vec<f64>,
) {
    let iterations = pairs.len();
    for _ in 0..iterations {
        // For every conversion pair attempt to update the conversion rate.
        for i in 0..pairs.len() {
            // Unpack the pair: from currency and to currency.
            let from = &pairs[i][0];
            let to = &pairs[i][1];
            let rate = rates[i];
            // If there's an entry for 'from', try relaxing 'to'
            if let Some(&from_value) = best.get(from) {
                // Get the current best value for 'to', or default to negative infinity.
                let to_entry = best.entry(to.clone()).or_insert(-1e18);
                // Update best conversion for currency 'to'
                *to_entry = (*to_entry).max(from_value * rate);
            }
            // For the reverse direction: if there's an entry for 'to', try relaxing 'from'
            if let Some(&to_value) = best.get(to) {
                // Get the current best value for 'from'
                let from_entry = best.entry(from.clone()).or_insert(-1e18);
                // Update best conversion for currency 'from' using the reverse rate.
                *from_entry = (*from_entry).max(to_value / rate);
            }
        }
    }
}

/// Computes the maximum amount achievable starting with an initial currency by performing
/// a series of currency conversions. The algorithm applies the Bellman-Ford relaxation on
/// two sets of currency conversion pairs consecutively.
fn max_amount(
    initial_currency: &str,
    pairs1: &Vec<Vec<String>>,
    rates1: &Vec<f64>,
    pairs2: &Vec<Vec<String>>,
    rates2: &Vec<f64>,
) -> f64 {
    // Keep track of the best conversion rates for each currency.
    let mut best: HashMap<String, f64> = HashMap::new();
    best.insert(initial_currency.to_string(), 1.0);
    
    // Apply the relaxation process on both sets of conversion pairs.
    bellman(&mut best, pairs1, rates1);
    bellman(&mut best, pairs2, rates2);
    
    // Return the best achievable amount in the initial currency.
    *best.get(initial_currency).unwrap_or(&0.0)
}

fn main() -> io::Result<()> {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // Read the initial currency from the first line.
    let initial_currency = if let Some(line) = reader.next() {
        line?.trim().to_string()
    } else {
        return Ok(());
    };

    // Read number of conversion pairs for the first set.
    let n1_line = if let Some(line) = reader.next() {
        line?
    } else {
        return Ok(());
    };
    let n1: usize = n1_line.trim().parse().expect("Failed to parse n1");
    
    // Parse the first set: each conversion pair consists of two currencies and a rate.
    let mut pairs1: Vec<Vec<String>> = Vec::with_capacity(n1);
    let mut rates1: Vec<f64> = Vec::with_capacity(n1);
    for _ in 0..n1 {
        // Read a line and split it into parts.
        let line = if let Some(line) = reader.next() {
            line?
        } else {
            break;
        };
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 3 {
            continue;
        }
        pairs1.push(vec![tokens[0].to_string(), tokens[1].to_string()]);
        rates1.push(tokens[2].parse::<f64>().expect("Failed to parse rate1"));
    }
    
    // Read number of conversion pairs for the second set.
    let n2_line = if let Some(line) = reader.next() {
        line?
    } else {
        return Ok(());
    };
    let n2: usize = n2_line.trim().parse().expect("Failed to parse n2");
    
    // Parse the second set: each conversion pair consists of two currencies and a rate.
    let mut pairs2: Vec<Vec<String>> = Vec::with_capacity(n2);
    let mut rates2: Vec<f64> = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = if let Some(line) = reader.next() {
            line?
        } else {
            break;
        };
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 3 {
            continue;
        }
        pairs2.push(vec![tokens[0].to_string(), tokens[1].to_string()]);
        rates2.push(tokens[2].parse::<f64>().expect("Failed to parse rate2"));
    }
    
    // Compute the maximum amount achievable in the initial currency.
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    
    // Write the result to stdout with fixed precision of 5 decimal places.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{:.5}", result)?;
    
    Ok(())
}