use crate::data_analysis::{errors, performance, print_all_logs, request_summary};
use crate::logfile_parser::{read, Log};
use std::io;
use std::io::BufRead;

mod data_analysis;
mod logfile_parser;
mod unit_tests;

// Since requests are stated that only to be GET,POST,PUT,DELETE
// I added an undefined state where there are errors in the request type.
// I got an error there since I forgot to derive Debug

// Disclaimer: Before this project, the biggest project I did is implementation of builder pattern, and implementation of structs etc. (I just jumped to the multithreading since I found an interesting problem). I even use enums in this project first time. ()
// I have found that builder pattern is a good match for that logfile project, since I think it is valuable to be maintainable, so with builder pattern we can change the structure of the logs easier.
// So, I am using my old builder pattern trial project as a starting template, with changes.
// This might reduce the performance since we are calling more functions per log, but generally IO is the real speed limiter when we are accessing files. So the performance affect might be ignorable.
fn main() {
    loop {
        // Read the user input
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        // Remove the newline character
        input.pop();
        // if nothing as input, ignore
        match input.as_str() {
            "" => continue,
            _ => {}
        };
        // Call your function and match the result
        let logs = match read(&input) {
            Ok(logs) => logs, // Return the logs vector from the match
            Err(e) => {
                println!("Error: {}", e);
                continue; // Skip the rest of the loop and ask for another input
            }
        };
        println!("{:?}", logs);
        println!(
            "Logfile is successfully selected, please select the operation to do: \n \
        Operations: 'Summary', 'Errors' , 'Performance', 'List_ALl'
        1 for Summary: how many times each type of request occurred.
        2 for Errors: List all of the errors group by endpoint url.
        3 for Performance Metrics: Average response time for each endpoint.
        4 to print all logs
        "
        );

        loop {
            let mut selectedcommand = String::new();
            io::stdin().lock().read_line(&mut selectedcommand).unwrap();
            selectedcommand.pop();
            match selectedcommand.as_str() {
                //since we are just investigating and analysing without changing the actual data, we just passed the address reference of logs to the functions.
                // In short, compiler prevented me to change the actual data accidentally inside my functions.
                "1" => request_summary(&logs),
                "2" => errors(&logs),
                "3" => performance(&logs),
                "4" => print_all_logs(&logs),
                // give a feedback and skip the remaining lines, to ask  a new command.
                _ => {
                    println!("invalid input, please type 1, 2 or 3.");
                    continue;
                }
            }
        }
    }
}
