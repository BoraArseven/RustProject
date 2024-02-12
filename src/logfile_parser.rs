use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

//for simplicity, for the sake of both users and developers, I made this code use only the logfiles that are inside the project directory. Thus,
// Users will just type the name of the file.

pub fn read(path: &str) -> std::io::Result<()> {
    // Get the path to the logfile
    let logfile_path = get_logfile_path(path);
    let f = File::open(logfile_path)?;
    let reader = BufReader::new(f);

    for line_result in reader.lines() {
        let line = line_result?;
        let mut start = 0;
        let mut end = 0;
        // in this format, the first white space is dividing the timestamp into 2. I don't want it so I ignore the first space with that.
        // maybe I could merge them together later, so I could pay this for computation later. But it is just a boolean check. Otherwise it will be string merge which is more costly.
        let mut ignorefirstwhitespace :bool =  true;
        for (i, c) in line.char_indices() {
            if c.is_whitespace() {
                if ignorefirstwhitespace == false {

                if start < end {
                    let term = &line[start..end];
                    print!("{} ", term);
                }


                start = i + 1;
                }else { ignorefirstwhitespace = false }
            }
            end = i + 1;
        }

        if start < end {
            let term = &line[start..end];
            println!("Term: {}", term);
        }
    }
    Ok(())
}
fn get_logfile_path(filename: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap(); // Get the current directory
    path.push(filename); // Append the filename to the current directory path
    path
}