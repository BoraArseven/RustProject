extern crate chrono;
use crate::data_analysis::{errors, performance, print_all_logs, request_summary};
use crate::logfile_parser::read;
use std::fmt::Debug;

use std::io;
use std::io::BufRead;

mod data_analysis;
mod logfile_parser;

// Since requests are stated that only to be GET,POST,PUT,DELETE
// I added an undefined state where there are errors in the request type.
// I got an error there since I forgot to derive Debug

// Disclaimer: Before this project, the biggest project I did is implementation of builder pattern, and implementation of structs etc. (I just jumped to the multithreading since I found an interesting problem). I even use enums in this project first time. ()
// I have found that builder pattern is a good match for that logfile project, since I think it is valuable to be maintainable, so with builder pattern we can change the structure of the logs easier.
// So, I am using my old builder pattern trial project as a starting template, with changes.
// This might reduce the performance since we are calling more functions per log, but generally IO is the real speed limiter when we are accessing files. So the performance affect might be ignorable.
//From Rust Documentation: OS threads are suitable for a small number of tasks, since threads come with CPU and memory overhead.
// Spawning and switching between threads is quite expensive as even idle threads consume system resources.
// A thread pool library can help mitigate some of these costs, but not all. However, threads let you reuse existing synchronous code
// without significant code changes—no particular programming model is required. In some operating systems,
// you can also change the priority of a thread, which is useful for drivers and other latency sensitive applications.
//
// Async provides significantly reduced CPU and memory overhead, especially for workloads with a large amount of IO-bound tasks, such as servers and databases. All else equal, you can have orders of magnitude more tasks than OS threads, because an async runtime uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks. However, async Rust results in larger binary blobs due to the state machines generated from async functions and since each executable bundles an async runtime.
#[tokio::main]
async fn main() {
    loop {
        println!(
            "Please enter the name of the project directory, for example 'log.txt', './log.txt'"
        );
        // Read the user input
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        // Remove the newline character
        input.pop();
        // if nothing as input, ignore and ask again.
        match input.as_str() {
            "" => continue,
            _ => {}
        };

        // Call your function and match the result
        let logs = match read(&input) {
            Ok(logs) => logs.0, // Return the logs vector from the match
            Err(e) => {
                println!("Error: {}", e);
                continue; // Skip the rest of the loop and ask for another input
            }
        };
        let malformedlogs = match read(&input) {
            Ok(logs) => logs.1, // Return the logs vector from the match
            Err(e) => {
                println!("Error: {}", e);
                continue; // Skip the rest of the loop and ask for another input
            }
        };
        println!(
            "Log file is successfully selected, before processing, please select the output type "
        );
        let mut output_file_type = String::new();
        let mut command: u8;
        loop {
            println!("Type 1 for txt output, 2 for csv, 3 to abort");
            io::stdin().lock().read_line(&mut output_file_type).unwrap();
            // Remove the newline character
            output_file_type = output_file_type.trim().parse().unwrap();
            // if nothing as input, ignore and ask again.
            match output_file_type.as_str() {
                "" => continue,
                _ => {}
            };
            // Trim the input strings and parse the command
            output_file_type = output_file_type.trim().parse().unwrap();
            command = output_file_type
                .parse::<u8>()
                .expect("error when parsing your command");

            // Break the loop if the command is 3
            match command {
                1 => break,
                2 => break,
                _ => continue,
            }
        }

        println!("please select the operation to do: ");

        loop {
            println!(
                " Operations: 'Summary', 'Errors' , 'Performance', 'List_ALl'
        1 for Summary: how many times each type of request occurred.
        2 for Errors: List all of the errors group by endpoint url.
        3 for Performance Metrics: Average response time for each endpoint.
        4 for printing all logs
        5 for printing all malformed logs
        "
            );

            let mut selectedcommand = String::new();
            io::stdin().lock().read_line(&mut selectedcommand).unwrap();
            selectedcommand.pop();
            // Select an option to analyse logs.
            match selectedcommand.as_str() {
                //since we are just investigating and analysing without changing the actual data, we just passed the address reference of logs to the functions.
                // In short, compiler prevented me to change the actual data accidentally inside my functions.
                "1" => request_summary(&logs, command).await, // wrap the value in a Box and cast it to a trait object
                "2" => errors(&logs, command).await, // wrap the value in a Box and cast it to a trait object
                "3" => performance(&logs, command).await, // wrap the value in a Box and cast it to a trait object
                "4" => print_all_logs(&logs), // wrap the value in a Box and cast it to a trait object
                "5" => {
                    for log in &malformedlogs {
                        println!("{:?}", log);
                    }
                }
                _ => {
                    println!("invalid input, please type 1, 2 or 3.");
                    continue;
                }
            };
            // give a feedback and skip the remaining lines, to ask  a new command.
        }
    }
}
