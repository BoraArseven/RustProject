use crate::logfile_parser::Log;
use crate::logfile_parser::Request;
// I might be able to use multithreading since our references are not mutable since we are not reading.
pub(crate) fn request_summary(logs: &Vec<Log>) {
    //[get,post,delete,put] in order.
    let mut counts = [0i64; 5];

    for log in logs {
        // Call the get method on the log reference
        let request_type = log.get_request_type();
        match request_type {
            Request::GET => counts [0]+=1,
            Request::POST => counts [1] +=1,
            Request::DELETE => counts [2] +=1,
            Request::PUT => counts [3] += 1,
            Request::UNDEFINED => counts[4] +=1,
            // If no type is defined, safe default undefined should act.
            _ => continue,
        }
    }
    println!("Counts are:
    GET: {:?}
    POST: {:?}
    DELETE: {:?}
    PUT: {:?}
    FAULTY: {:?}",

             counts[0], counts[1], counts[2], counts[3],counts[4])
}

pub(crate) fn errors(logs: &Vec<Log>) {}

pub(crate) fn performance(logs: &Vec<Log>) {


}


pub(crate) fn print_all_logs(logs: &Vec<Log>) {
    for log in logs {
        // Use println! with {:?} to print each log
        println!("{:?}", log);
    }
}
