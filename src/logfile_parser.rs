use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::{env, io};

//for simplicity, for the sake of both users and developers, I made this code use only the logfiles that are inside the project directory. Thus,
// Users will just type the name of the file.
#[derive(Debug, Clone)]
pub enum Request {
    GET,
    POST,
    PUT,
    DELETE,
    UNDEFINED,
}
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
            .set_time_stamp(if terms.len() > 1 {
                Some([terms[0], terms[1]].join(" "))
            } else {
                None
            })
            .set_request_type(match terms.get(2) {
                Some(&"GET") => Some(Request::GET),
                Some(&"POST") => Some(Request::POST),
                Some(&"DELETE") => Some(Request::DELETE),
                Some(&"PUT") => Some(Request::PUT),
                // If no type is defined, safe default undefined should act.
                _ => None,
            })
            .set_endpoint_url(terms.get(3).map(|s| s.to_string()))
            .set_status_code(terms.get(4).and_then(|s| s.parse::<i16>().ok()))
            .set_response_time(terms.get(5).and_then(|s| s.parse::<i32>().ok()))
            .build();
        logs.push(entry);
    }
    //I am not confident with this line, I just found on the internet, I was just tried (Ok,logs)
    Ok(logs)
}
pub fn get_logfile_path(filename: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap(); // Get the current directory
    path.push(filename); // Append the filename to the current directory path
    path
}
// derive debug is to be able to print with :?
#[derive(Debug)]
pub struct Log {
    timestamp: String,
    request_type: Request,
    endpoint_url: String,
    // I could use unsigned but I wasn't sure
    status_code: i16,
    response_time: i32,
}
struct LogBuilder {
    timestamp: String,
    request_type: Request,
    endpoint_url: String,
    // Since there are a lot of status codes, I will use integer and check a range.
    status_code: i16,
    response_time: i32,
}

impl Log {
    // Change the return type to &str
    pub fn get_timestamp(&self) -> &str {
        // Return a reference to the field
        &self.timestamp
    }
    pub fn get_status_code(&self) -> &i16 {
        &self.status_code
    }
    pub fn get_endpoint_url(&self) -> &String {
        &self.endpoint_url
    }
    pub fn get_request_type(&self) -> &Request {
        &self.request_type
    }
    pub fn get_response_time(&self) -> &i32 {
        &self.response_time
    }
}

impl LogBuilder {
    pub fn new(/* ... */) -> LogBuilder {
        // Set the default values for log, which can be assumed as "SAFE DEFAULT" so if one of the fields are empty, it will automatically replaces it with safe defaults.
        LogBuilder {
            timestamp: "None".parse().unwrap(),
            request_type: Request::UNDEFINED,
            endpoint_url: "Error".parse().unwrap(),
            status_code: -1,
            response_time: -1,
        }
    }
    //setters for each field
    fn set_time_stamp(&mut self, timestamp: Option<String>) -> &mut Self {
        self.timestamp = timestamp.unwrap();
        self
    }
    fn set_request_type(&mut self, request_type: Option<Request>) -> &mut Self {
        self.request_type = request_type.unwrap();
        self
    }
    fn set_endpoint_url(&mut self, endpoint_url: Option<String>) -> &mut Self {
        self.endpoint_url = endpoint_url.unwrap();
        self
    }
    fn set_status_code(&mut self, status_code: Option<i16>) -> &mut Self {
        self.status_code = status_code.unwrap();
        self
    }
    fn set_response_time(&mut self, response_time: Option<i32>) -> &mut Self {
        self.response_time = response_time.unwrap();
        self
    }
    // I am not sure about clone(), maybe it might be a bad practice, I need a feedback here.
    fn build(&mut self) -> Log {
        Log {
            request_type: self.request_type.clone(),
            timestamp: self.timestamp.clone(),
            endpoint_url: self.endpoint_url.clone(),
            status_code: self.status_code.clone(),
            response_time: self.response_time.clone(),
        }
    }
}
