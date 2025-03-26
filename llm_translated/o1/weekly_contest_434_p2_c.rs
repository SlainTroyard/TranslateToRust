use std::io::{self, Read};
use std::cmp::Ordering;

/// Converts a string to an integer by extracting only its digit characters.
/// Example: "abc123" becomes 123.
fn str_to_num(s: &str) -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + (c as i32 - '0' as i32);
        }
    }
    num
}

/// Extracts the number portion from an ID string (e.g., from "id123" returns 123).
/// Returns -1 if no digit is found.
fn extract_id_number(id_str: &str) -> i32 {
    if let Some(pos) = id_str.chars().position(|c| c.is_ascii_digit()) {
        str_to_num(&id_str[pos..])
    } else {
        -1
    }
}

/// The core function that replicates the logic of countMentions from the C code.
/// number_of_users: number of users
/// events: a 2D vector where each row has exactly 3 strings: [EventType, Time, Details]
/// Returns a vector of mention counts for each user in the range [0..number_of_users).
fn count_mentions(number_of_users: usize, events: &[Vec<String>]) -> Vec<i32> {
    let event_count = events.len();

    // Extract times from the second column (events[i][1])
    let mut time_arr: Vec<i32> = events.iter().map(|row| str_to_num(&row[1])).collect();

    // Initialize an ordering array [0, 1, 2, ..., event_count - 1]
    let mut order: Vec<usize> = (0..event_count).collect();

    // Bubble sort based on:
    // 1) ascending time
    // 2) if times are equal, 'O' event precedes 'M' event
    for i in (1..event_count).rev() {
        for j in 0..i {
            let current_idx = order[j];
            let next_idx = order[j + 1];
            let current_time = time_arr[current_idx];
            let next_time = time_arr[next_idx];

            // Compare times
            if next_time < current_time
                || (next_time == current_time
                    && events[next_idx][0].chars().next() == Some('O'))
            {
                order.swap(j, j + 1);
            }
        }
    }

    // "online" array tracks the time until which each user remains "online"
    // Initially all 0 (meaning offline)
    let mut online = vec![0; number_of_users];

    // "mention" array holds the mention counts for each user
    let mut mention = vec![0; number_of_users];

    // Process each event in the sorted order
    for &sorted_idx in &order {
        let event_type = events[sorted_idx][0].chars().next().unwrap_or('\0');
        let event_time_str = &events[sorted_idx][1];
        let event_details = &events[sorted_idx][2];

        match event_type {
            // 'M' events
            'M' => {
                let third_col_first_char = event_details.chars().next().unwrap_or('\0');
                match third_col_first_char {
                    // 'A' => mention all users
                    'A' => {
                        for j in 0..number_of_users {
                            mention[j] += 1;
                        }
                    }
                    // 'H' => mention all offline users or users whose online time has expired
                    'H' => {
                        let time = str_to_num(event_time_str);
                        for j in 0..number_of_users {
                            if online[j] == 0 {
                                mention[j] += 1;
                            } else if online[j] <= time {
                                online[j] = 0;
                                mention[j] += 1;
                            }
                        }
                    }
                    // Otherwise, parse user IDs from the string and increment their mention count
                    _ => {
                        // Split the details by spaces to get each ID segment
                        for t_id in event_details.split_whitespace() {
                            let user_id = extract_id_number(t_id);
                            if user_id >= 0 && (user_id as usize) < number_of_users {
                                mention[user_id as usize] += 1;
                            }
                        }
                    }
                }
            }
            // 'O' events
            'O' => {
                // Mark a user as online for time + 60
                let user_id = extract_id_number(event_details);
                if user_id >= 0 && (user_id as usize) < number_of_users {
                    online[user_id as usize] = str_to_num(event_time_str) + 60;
                }
            }
            _ => {
                // Ignore any other event types (not expected by the original problem statement)
            }
        }
    }

    // Return the mention counts
    mention
}

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading input");
        return;
    }

    let mut tokens = input.split_whitespace();
    // Parse number_of_users and events_size
    let number_of_users: usize = match tokens.next().and_then(|x| x.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("Memory allocation failed");
            return;
        }
    };
    let events_size: usize = match tokens.next().and_then(|x| x.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("Memory allocation failed");
            return;
        }
    };

    // Read the event data: each event has 3 strings
    let mut events = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let mut row = Vec::with_capacity(3);
        for _ in 0..3 {
            if let Some(txt) = tokens.next() {
                row.push(txt.to_string());
            } else {
                eprintln!("Memory allocation failed");
                return;
            }
        }
        events.push(row);
    }

    // Compute mentions
    let result = count_mentions(number_of_users, &events);

    // Output the result in the same format as the C code
    print!("Mentions: ");
    for val in &result {
        print!("{} ", val);
    }
    println!();
}