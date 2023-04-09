use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone};
use chrono::LocalResult;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Default behavior: print current Unix time and time
        let current_time = SystemTime::now();
        let unix_timestamp = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let current_datetime: DateTime<Utc> = Utc::now();
        println!("Current Unix timestamp: {}", unix_timestamp);
        println!("Current time: {}", current_datetime);
    } else {
        let subcommand = &args[1];

        if subcommand == "-c" {
            // Option: -c (current time)
            let current_time = SystemTime::now();
            let unix_timestamp = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
            let current_datetime: DateTime<Utc> = Utc::now();
            println!("Current Unix timestamp: {}", unix_timestamp);
            println!("Current time: {}", current_datetime);
        } else if subcommand == "-d" {
            // Option: -d (time difference)
            if args.len() < 4 {
                println!("Error: missing arguments. Usage: cargo run -d -u <unix_timestamp>");
                return;
            }

            let unix_timestamp = args[3].parse::<i64>().unwrap();
            let datetime = Utc.timestamp_opt(unix_timestamp, 0);
            match datetime {
                LocalResult::Single(datetime) => {
                    let current_datetime: DateTime<Utc> = Utc::now();
                    let time_difference = current_datetime.signed_duration_since(datetime);

                    if args[2] == "-u" {
                        // Option: -u (display time difference in Unix seconds)
                        println!("Time difference (Unix seconds): {}", time_difference.num_seconds());
                    } else {
                        // Default behavior: display time difference as a formatted string
                        println!("Time difference: {}", time_difference);
                    }
                },
                _ => {
                    println!("Error: invalid Unix timestamp");
                }
            }
        } else {
            // Invalid subcommand
            println!("Error: invalid subcommand. Usage: cargo run [-c | -d -u <unix_timestamp>]");
        }
    }
}
