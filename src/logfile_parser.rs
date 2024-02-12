use crate::{Log, LogBuilder, Request};
use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

//for simplicity, for the sake of both users and developers, I made this code use only the logfiles that are inside the project directory. Thus,
// Users will just type the name of the file.

pub fn read(path: &str) -> io::Result<Vec<Log>> {
    let mut logs: Vec<Log> = Vec::new();
    // Get the path to the logfile
    let logfile_path = get_logfile_path(path);
    let f = File::open(logfile_path)?;
    let reader = BufReader::new(f);

    for line_result in reader.lines() {
        let line = line_result?;
        let terms: Vec<&str> = line.split_whitespace().collect();

        // Assuming LogBuilder and Log types are defined elsewhere
        let entry: Log = LogBuilder::new()
            .settimestamp([terms[0], terms[1]].join(" "))
            .setrequest(match terms[2] {
                "GET" => Request::GET,
                "POST" => Request::POST,
                "DELETE" => Request::DELETE,
                "PUT" => Request::PUT,
                _ => Request::UNDEFINED,
            })
            .setendpoint_url(terms[3].to_string()) // You should use `to_string()` instead of `unwrap()`
            .setstatuscode(terms[4].parse::<i16>().unwrap())
            .setresponsetime(terms[5].parse::<i32>().unwrap()) // You should use `to_string()` instead of `unwrap()`
            .build();
        logs.push(entry);
    }
    //I am not confident with this line, I just found on the internet, I was just tried (Ok,logs)
    Ok(logs)
}
fn get_logfile_path(filename: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap(); // Get the current directory
    path.push(filename); // Append the filename to the current directory path
    path
}
