use std::cmp::Ordering;
use std::io::{self, Read};

/// Converts a string slice into a number by accumulating all its digit characters.
/// This mimics the C function `str_to_num` by iterating over each char.
fn str_to_num(s: &str) -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + (c as i32 - '0' as i32);
        }
    }
    num
}

/// Extracts the numeric part from an id string (e.g., "id123" -> 123).
/// Returns -1 if no digit is found.
fn extract_id_number(s: &str) -> i32 {
    if let Some(pos) = s.find(|c: char| c.is_ascii_digit()) {
        str_to_num(&s[pos..])
    } else {
        -1
    }
}

/// Processes events and counts "mentions" for each user.
/// `number_of_users`: total number of users
/// `events`: a vector of events, where each event is a vector of three strings:
///           [event_type, time, third_field]
///
/// The algorithm follows these steps:
/// 1. Build an array of times from events.
/// 2. Create an order vector (indices) and sort it by time, and for events with equal time,
///    an event with type 'O' comes before other event types.
/// 3. Process each event in the sorted order:
///    - For 'M' (mention) events:
///         * If third_field starts with 'A': add 1 mention to all users.
///         * If third_field starts with 'H': compare the current time to each online threshold:
///              - If the user is offline (0), add 1 mention.
///              - If the user was online but the online time threshold is reached, set offline and add 1 mention.
///         * Otherwise, split the third_field by whitespace
///              and extract the id number from each token and add a mention.
///    - For 'O' (online) events:
///         * Extract user id from the third field and set that user's online time to (time + 60).
fn count_mentions(number_of_users: usize, events: &[Vec<String>]) -> Vec<i32> {
    let events_size = events.len();

    // Build an array of times for each event by converting events[i][1] to a number.
    let mut time_arr = vec![0; events_size];
    for i in 0..events_size {
        time_arr[i] = str_to_num(&events[i][1]);
    }

    // Create an array with indices representing each event.
    let mut order: Vec<usize> = (0..events_size).collect();

    // Sort the indices by the event times in ascending order.
    // For events with the same time, if one event's type is 'O', it should come before.
    order.sort_by(|&i, &j| {
        let ti = time_arr[i];
        let tj = time_arr[j];
        match ti.cmp(&tj) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let event_i = events[i][0].chars().next().unwrap();
                let event_j = events[j][0].chars().next().unwrap();
                // If the second event is 'O' and the first is not, then the second should come first.
                if event_j == 'O' && event_i != 'O' {
                    Ordering::Greater
                } else if event_i == 'O' && event_j != 'O' {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        }
    });

    // Array to keep track of online state for each user.
    let mut online = vec![0; number_of_users];
    // Array to accumulate mention counts for each user.
    let mut mention = vec![0; number_of_users];

    // Process events in sorted order.
    for &i in &order {
        let event_type = events[i][0].chars().next().unwrap();
        if event_type == 'M' {
            // "M" event indicates a mention event.
            let third_field_first = events[i][2].chars().next().unwrap();
            if third_field_first == 'A' {
                // If the third field starts with 'A', add 1 mention to every user.
                for j in 0..number_of_users {
                    mention[j] += 1;
                }
            } else if third_field_first == 'H' {
                // If it starts with 'H', compare the online state with the event time.
                let time = str_to_num(&events[i][1]);
                for j in 0..number_of_users {
                    if online[j] == 0 {
                        mention[j] += 1;
                    } else if online[j] <= time {
                        // User's online period has expired.
                        online[j] = 0;
                        mention[j] += 1;
                    }
                }
            } else {
                // Otherwise, the third field is assumed to contain space-separated IDs.
                // Split by whitespace and extract each ID.
                for token in events[i][2].split_whitespace() {
                    let user_id = extract_id_number(token);
                    if user_id >= 0 && (user_id as usize) < number_of_users {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if event_type == 'O' {
            // "O" event indicates an online event.
            let user_id = extract_id_number(&events[i][2]);
            if user_id >= 0 && (user_id as usize) < number_of_users {
                // Set the online period end time to (current time + 60).
                online[user_id as usize] = str_to_num(&events[i][1]) + 60;
            }
        }
    }

    mention
}

fn main() -> io::Result<()> {
    // Read the whole input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Ensure that there are at least two tokens for numberOfUsers and eventsSize.
    if tokens.len() < 2 {
        eprintln!("Error: insufficient input provided.");
        return Ok(());
    }

    // Parse the number of users and the number of events.
    let number_of_users: usize = tokens[0].parse().unwrap_or_else(|_| {
        eprintln!("Error: invalid number of users.");
        std::process::exit(1);
    });
    let events_size: usize = tokens[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: invalid events size.");
        std::process::exit(1);
    });

    // Read events.
    // Each event consists of 3 strings, so we need exactly tokens[2 .. 2 + events_size * 3]
    if tokens.len() < 2 + events_size * 3 {
        eprintln!("Error: insufficient event data provided.");
        std::process::exit(1);
    }

    let mut events = Vec::with_capacity(events_size);
    let mut idx = 2; // start reading after number_of_users and events_size
    for _ in 0..events_size {
        let mut event = Vec::with_capacity(3);
        // Each event has 3 fields: type, time, and the third field.
        event.push(tokens[idx].to_string());
        event.push(tokens[idx + 1].to_string());
        event.push(tokens[idx + 2].to_string());
        events.push(event);
        idx += 3;
    }

    // Compute the mentions array.
    let mention = count_mentions(number_of_users, &events);

    // Print the output in the exact same format:
    // "Mentions: " followed by the mention count for each user separated by a space.
    print!("Mentions: ");
    for count in &mention {
        print!("{} ", count);
    }
    println!();

    Ok(())
}