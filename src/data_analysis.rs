extern crate chrono;
extern crate tokio;
use crate::logfile_parser::Request;
use crate::logfile_parser::{get_logfile_path, Log};
use chrono::Utc;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
struct Count {
    get: i64,
    post: i64,
    delete: i64,
    put: i64,
    undefined: i64,
}

// I might be able to use multithreading since our references are not mutable since we are not reading.
pub(crate) async fn request_summary(logs: &Vec<Log>, command: u8) {
    //[get,post,delete,put] in order.
    let mut counts = Count {
        get: 0,
        post: 0,
        delete: 0,
        put: 0,
        undefined: 0,
    };

    for log in logs {
        // Call the get method on the log reference
        let request_type = log.get_request_type();
        match request_type {
            Request::GET => counts.get += 1,
            Request::POST => counts.post += 1,
            Request::DELETE => counts.delete += 1,
            Request::PUT => counts.put += 1,
            Request::UNDEFINED => counts.undefined += 1,
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
        counts.get, counts.post, counts.delete, counts.put, counts.undefined
    );
    print_to_file(&counts, command).await;
}

async fn print_to_file(mut counts: &Count, command: u8) {
    let dt = Utc::now();
    // Convert it to a timestamp in seconds
    let timestamp: i64 = dt.timestamp();
    // Convert it to a string
    let timestamp_as_string = timestamp.to_string();
    let mut selected_filename = format!("{}{}", "Request_Summary", timestamp_as_string);

    let path = match command {
        1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
        2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
        _ => panic!("Invalid command"),
    };

    // Create a file with the given path
    let mut file = File::create(&path).await.expect("Failed to create file");
    // Create a line with the counts
    let line = match command {
        1 => format!(
            "Get: {}\nPost: {}\nDelete: {}\nPUT: {}\nFAULTY: {}",
            counts.get, counts.post, counts.delete, counts.put, counts.undefined
        ),
        2 => format!(
            "GET,POST,DELETE,PUT,FAULTY\n{},{},{},{},{}",
            counts.get, counts.post, counts.delete, counts.put, counts.undefined
        ),
        _ => panic!("Invalid_Command_When_Writing_File"),
    };

    // Write the line to the file asynchronously
    // Use catch_unwind to catch any panics
    let data = line.as_bytes(); // Move the data outside the closure
    let result = std::panic::catch_unwind(|| async {
        data // Pass the data as an argument
    });
    file.write_all(data) // Write the data to the file after catching the panic
        .await
        .expect("Could not write to file");
    // Handle the result
    match result {
        Ok(_) => println!("Written to the file successfully"),
        Err(e) => {
            // You can do whatever you want with the error here
            // For example, log it or retry the operation
            eprintln!("An error occurred while writing to the file: {:?}", e);
        }
    }
}

pub(crate) async fn errors(logs: &Vec<Log>, command: u8) {
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
            // keys are endpoint_url so this means we grouped the data by endpoint_url.
            results.entry(endpoint_url).or_insert(Vec::new()).push(log);
        }
    }
    let dt = Utc::now();
    // Convert it to a timestamp in seconds
    let timestamp: i64 = dt.timestamp();
    // Convert it to a string
    let timestamp_as_string = timestamp.to_string();
    let selected_filename = format!("{}{}", "errors", timestamp_as_string);

    // Create a path with the given filename and the appropriate extension
    let path = match command {
        1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
        2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
        _ => panic!("Invalid command while writing to file"),
    };

    // Create a file with the given path
    let file = File::create(&path).await.expect("Failed to create file");

    // Create a tokio BufWriter with the file
    let mut buf_writer = tokio::io::BufWriter::new(file);

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
            .await // Use await here
            .expect("Failed to write to buffer");

        // Iterate over the logs and write the details to the buffer
        for log in logs {
            let line = match command {
                1 => format!(
                    "Timestamp: {:?}, Request Type: {:?}, Status Code: {:?}, Response Time: {:?}\n",
                    log.get_timestamp(),
                    log.get_request_type(),
                    log.get_status_code(),
                    log.get_response_time()
                ),
                2 => format!(
                    "{:?},{:?},{:?},{:?}\n",
                    log.get_timestamp(),
                    log.get_request_type(),
                    log.get_status_code(),
                    log.get_response_time()
                ),
                _ => continue,
            };
            // I think this is not the best way to do that, but my skill level and time is not enough to stack all of the lines at once and give it to the bufwriter.
            // Maybe when I update and maintain the code I can do that optimization.
            buf_writer
                .write_all(line.as_bytes())
                .await // Use await here
                .expect("Failed to write to buffer");
        }

        // Add a new line after each endpoint URL
        buf_writer
            .write_all(b"\n")
            .await // Use await here
            .expect("Failed to write to buffer");
    }
}

pub(crate) async fn performance(logs: &Vec<Log>, command: u8) {
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
    let dt = Utc::now();
    // Convert it to a timestamp in seconds
    let timestamp: i64 = dt.timestamp();
    // Convert it to a string
    let timestamp_as_string = timestamp.to_string();
    let selected_filename = format!("{}{}", "Performance", timestamp_as_string);
    // Trim the input strings and parse the command
    // Create a path with the given filename and the appropriate extension
    let path = match command {
        1 => get_logfile_path(&format!("{}{}", selected_filename, ".txt")),
        2 => get_logfile_path(&format!("{}{}", selected_filename, ".csv")),
        _ => panic!("Invalid command, this command must not be able to reached here."),
    };

    // Create a file with the given path
    let file = File::create(&path).await.expect("Failed to create file"); // Use tokio::fs::File and await

    // Create a tokio BufWriter with the file
    let mut buf_writer = tokio::io::BufWriter::new(file); // Use tokio::io::BufWriter

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
            .await // Use await here
            .expect("Failed to write to buffer");
    }

    // Flush the buffer to write data to the file
    buf_writer.flush().await.expect("Failed to flush buffer"); // Use await here

    println!("File created successfully!");
}

pub(crate) fn print_all_logs(logs: &Vec<Log>) {
    for log in logs {
        // Use println! with {:?} to print each log
        println!("{:?}", log);
    }
} /*
  #[cfg(test)]
  mod tests {
      use crate::data_analysis::{errors, request_summary};
      use crate::logfile_parser::{LogBuilder, Request};

      #[test]
      fn test_error_count() {
          // Test vector is created by github copilot.
          let logs = vec![
              LogBuilder::new()
                  .set_time_stamp(Some("2021-01-01 00:00:00".to_string()))
                  .set_endpoint_url(Some("https://www.google.com".to_string()))
                  .set_status_code(Some(200))
                  .set_response_time(Some(100))
                  .build(),
              LogBuilder::new()
                  .set_time_stamp(Some("2021-01-01 00:00:00".to_string()))
                  .set_request_type(Some(Request::POST))
                  .set_endpoint_url(Some("https://www.google.com".to_string()))
                  .set_status_code(Some(404))
                  .set_response_time(Some(100))
                  .build(),
              LogBuilder::new()
                  .set_time_stamp(Some("2021-01-01 00:00:00".to_string()))
                  .set_request_type(Some(Request::GET))
                  .set_endpoint_url(Some("https://www.google.com".to_string()))
                  .set_status_code(Some(500))
                  .set_response_time(Some(100))
                  .build(),
              LogBuilder::new()
                  .set_time_stamp(Some("2021-01-01 00:00:00".to_string()))
                  .set_request_type(Some(Request::GET))
                  .set_endpoint_url(Some("https://www.google.com".to_string()))
                  .set_status_code(Some(200))
                  .set_response_time(Some(100))
                  .build(),
              LogBuilder::new()
                  .set_time_stamp(Some("2021-01-01 00:00:00".to_string()))
                  .set_request_type(Some(Request::GET))
                  .set_endpoint_url(Some("https://www.google.com".to_string()))
                  .set_status_code(Some(404))
                  .set_response_time(Some(100))
                  .build(),
          ];
      let counts : [i64;5] = [4,1,0,0,0];
          assert_eq!(request_summary(&logs, 0), counts);
      }
  }
  */
