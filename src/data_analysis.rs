use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, Write};

use crate::logfile_parser::Request;
use crate::logfile_parser::{get_logfile_path, Log};
// I might be able to use multithreading since our references are not mutable since we are not reading.
pub(crate) fn request_summary(logs: &Vec<Log>) {
    //[get,post,delete,put] in order.
    let mut counts = [0i64; 5];

    for log in logs {
        // Call the get method on the log reference
        let request_type = log.get_request_type();
        match request_type {
            Request::GET => counts[0] += 1,
            Request::POST => counts[1] += 1,
            Request::DELETE => counts[2] += 1,
            Request::PUT => counts[3] += 1,
            Request::UNDEFINED => counts[4] += 1,
            // If no type is defined, safe default undefined should act.
            _ => continue,
        }
    }
    println!(
        "Counts are:
    GET: {:?}
    POST: {:?}
    DELETE: {:?}
    PUT: {:?}
    FAULTY: {:?}",
        counts[0], counts[1], counts[2], counts[3], counts[4]
    );
    let mut selected_command = String::new();
    let mut selected_filename = String::new();
    loop {
        println!("Type 1 for txt output, 2 for csv, 3 to abort");
        io::stdin().lock().read_line(&mut selected_command).unwrap();
        println!("Please give a filename");
        io::stdin()
            .lock()
            .read_line(&mut selected_filename)
            .unwrap();

        // Trim the input strings and parse the command
        let selected_command = selected_command.trim();
        let selected_filename = selected_filename.trim();
        let command = selected_command
            .parse::<u8>()
            .expect("error when parsing your command");

        // Break the loop if the command is 3
        if command == 3 {
            break;
        }

        // Create a path with the given filename and the appropriate extension
        let path = match command {
            1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
            2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
            3 => break,
            _ => continue,
        };

        // Create a file with the given path
        let file = File::create(&path).expect("Failed to create file");
        let mut buf_writer = BufWriter::new(file);
        // Create a BufWriter with the filea
        let line = match command {
            1 => String::from(format!(
                "Get: {}\nPost: {}\nDelete: {}\nPUT: {}\nFAULTY: {}",
                counts[0], counts[1], counts[2], counts[3], counts[4]
            )),
            2 => String::from(format!(
                "GET,POST,DELETE,PUT,FAULTY\n{},{},{},{},{}",
                counts[0], counts[1], counts[2], counts[3], counts[4]
            )),
            _ => continue,
        };

        buf_writer
            .write(line.as_ref())
            .expect("Could not write to file");
        println!("Written to the file successfully");
        break;
    }
}

pub(crate) fn errors(logs: &Vec<Log>) {
    // Create an empty HashMap,
    let mut results = HashMap::new();

    // Iterate over the requests
    for log in logs {
        // Get the status code and the endpoint URL of the request
        let status_code = log.get_status_code();
        let endpoint_url = log.get_endpoint_url();

        // Check if the status code has error
        if status_code >= &400 && status_code <= &599 {
            // Push the log to the vector for the endpoint URL in the HashMap
            // If the key does not exist, insert a default value of an empty vector
            // keys are endpoint_url so this means we groupped the data by endpoint_url.
            results.entry(endpoint_url).or_insert(Vec::new()).push(log);
        }
    }
    let mut selected_command = String::new();
    let mut selected_filename = String::new();
    loop {
        println!("Type 1 for txt output, 2 for csv, 3 to abort");
        io::stdin().lock().read_line(&mut selected_command).unwrap();
        println!("Please give a filename");
        io::stdin()
            .lock()
            .read_line(&mut selected_filename)
            .unwrap();

        // Trim the input strings and parse the command
        let selected_command = selected_command.trim();
        let selected_filename = selected_filename.trim();
        let command = selected_command
            .parse::<u8>()
            .expect("error when parsing your command");

        // Break the loop if the command is 3
        if command == 3 {
            break;
        }

        // Create a path with the given filename and the appropriate extension
        let path = match command {
            1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
            2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
            3 => break,
            _ => continue,
        };

        // Create a file with the given path
        let file = File::create(&path).expect("Failed to create file");

        // Create a BufWriter with the file
        let mut buf_writer = BufWriter::new(file);

        // Iterate over the HashMap and write the data to the buffer
        for (endpoint_url, logs) in results.iter() {
            // First, since we are grouping according to endpoint url, for the first entity in result,
            // we create a line to show which endpoint the following logs belong.
            let line = match command {
                1 => format!("Endpoint URL: {:?}\n", endpoint_url),
                2 => format!("{:?},", endpoint_url),
                _ => continue,
            };
            buf_writer
                .write_all(line.as_bytes())
                .expect("Failed to write to buffer");

            // Iterate over the logs and write the details to the buffer
            for log in logs {
                let line = match command {
                    1 => format!("Timestamp: {:?}, Request Type: {:?}, Status Code: {:?}, Response Time: {:?}\n", log.get_timestamp(),
                                 log.get_request_type(), log.get_status_code(), log.get_response_time()),
                    2 => format!("{:?},{:?},{:?},{:?}\n", log.get_timestamp(), log.get_request_type(), log.get_status_code(), log.get_response_time()),
                    _ => continue,
                };
                // I think this is not the best way to do that, but my skill level and time is not enough to stack all of the lines at once and give it to the bufwriter.
                // Maybe when I update and maintain the code I can do that optimization.
                buf_writer
                    .write_all(line.as_bytes())
                    .expect("Failed to write to buffer");
            }

            // Add a new line after each endpoint URL
            buf_writer
                .write_all(b"\n")
                .expect("Failed to write to buffer");
        }

        // Flush the buffer to write data to the file
        buf_writer.flush().expect("Failed to flush buffer");

        println!("File created successfully!");
        break;
    }
    println!("{:?}", results);
}
pub(crate) fn performance(logs: &Vec<Log>) {
    // Create an empty HashMap
    let mut results = HashMap::new();

    // Iterate over the requests
    for log in logs {
        // Get the endpoint URL and the response time of the request
        let endpoint_url = log.get_endpoint_url();
        let response_time = log.get_response_time();

        // Get the entry for the endpoint URL in the HashMap, or insert a default value of (0, 0), which is as (response_time,count)
        let entry = results.entry(endpoint_url).or_insert((0, 0));

        // Increment the sum and the count of the response times for the endpoint URL
        entry.0 += response_time;
        entry.1 += 1;
    }

    // Iterate over the HashMap and calculate the average response time for each endpoint URL
    for (endpoint_url, (sum, count)) in results.iter_mut() {
        let division = *sum / *count;
        println!(
            "Endpoint: {:?}, Average Response Time: {:?}",
            endpoint_url, division
        )
    }
    let mut selected_command = String::new();
    let mut selected_filename = String::new();
    // I am not sure if it is good approach
    loop {
        println!("Type 1 for txt output, 2 for csv, 3 to abort");
        io::stdin().lock().read_line(&mut selected_command).unwrap();
        println!("Please give a filename");
        io::stdin()
            .lock()
            .read_line(&mut selected_filename)
            .unwrap();

        // Trim the input strings and parse the command
        let selected_command = selected_command.trim();
        let selected_filename = selected_filename.trim();
        let command = selected_command.parse::<u8>().expect("asd");

        // Break the loop if the command is 3
        if command == 3 {
            break;
        }

        // Create a path with the given filename and the appropriate extension
        let path = match command {
            1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
            2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
            3 => break,
            _ => continue,
        };

        // Create a file with the given path
        let file = File::create(&path).expect("Failed to create file");

        // Create a BufWriter with the file
        let mut buf_writer = BufWriter::new(file);

        // Iterate over the HashMap and write the data to the buffer
        for (endpoint_url, (sum, count)) in results.iter() {
            let division = *sum / *count;
            let line = match command {
                1 => format!(
                    "Endpoint: {:?}, Average Response Time: {:?}\n",
                    endpoint_url, division
                ),
                2 => format!("{:?},{:?}\n", endpoint_url, division),
                _ => continue,
            };
            buf_writer
                .write_all(line.as_bytes())
                .expect("Failed to write to buffer");
        }

        // Flush the buffer to write data to the file
        buf_writer.flush().expect("Failed to flush buffer");

        println!("File created successfully!");
        break;
    }
}

pub(crate) fn print_all_logs(logs: &Vec<Log>) {
    for log in logs {
        // Use println! with {:?} to print each log
        println!("{:?}", log);
    }
}
