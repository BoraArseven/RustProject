mod logfile_parser;
// Since requests are stated that only to be GET,POST,PUT,DELETE
// I added an undefined state where there are errors in the request type.
// I got an error there since I forgot to derive Debug
#[derive(Debug, Clone)]
enum Request {
    GET,
    POST,
    PUT,
    DELETE,
    UNDEFINED,
}

// Disclaimer: Before this project, the biggest project I did is implementation of builder pattern, and implementation of structs etc (I just jumped to the multithreading since I found an interesting problem). I even use enums in this project first time. ()
// I have found that builder pattern is a good match for that logfile project, since I think it is valuable to be maintainable, so with builder pattern we can change the structure of the logs easier.
// So, I am using my old builder pattern trial project as a starting template, with changes.
// This might reduce the performance since we are calling more functions per log, but generally IO is the real speed limiter when we are accessing files. So the performance affect might be ignorable.
fn main() {
    // I did not give a request type to my initial request on purpose, it gave the default UNDEFINED request since I stated it as a default. This is also good sign in case of faulty logs.
    let initiallog: Log = LogBuilder::new()
        .settimestamp("bora.arseven@gmail.com".parse().unwrap())
        .setstatuscode("404".parse().unwrap())
        .build();
    println!("{:?}", initiallog);
    println!("-----------------");
    logfile_parser::read("log.txt").expect("FILE CANNOT CANNOT BE READED: Please check the file path.");
}

#[derive(Debug)]
struct Log {
    timestamp: String,
    request_type: Request,
    endpoint_url: String,
    // I could use unsigned but I wasn't sure
    status_code: i16,
    responsetime: i32,
}
struct LogBuilder {
    timestamp: String,
    request_type: Request,
    endpoint_url: String,
    // Since there are a lot of status codes, I will use integer and check a range.
    status_code: i16,
    responsetime: i32,
}

impl Log {
    fn new(
        timestamp: String,
        request_type: Request,
        endpoint_url: String,
        status_code: i16,
        responsetime: i32,
    ) -> LogBuilder {
        LogBuilder {
            timestamp,
            request_type,
            endpoint_url,
            status_code,
            responsetime,
        }
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
            responsetime: -1,
        }
    }
    //setters for each field
    fn settimestamp(&mut self, timestamp: String) -> &mut Self {
        self.timestamp = timestamp;
        self
    }
    fn setrequest(&mut self, request_type: Request) -> &mut Self {
        self.request_type = request_type;
        self
    }
    fn setendpoint_url(&mut self, endpoint_url: String) -> &mut Self {
        self.endpoint_url = endpoint_url;
        self
    }
    fn setstatuscode(&mut self, status_code: i16) -> &mut Self {
        self.status_code = status_code;
        self
    }
    fn build(&mut self) -> Log {
        Log {
            request_type: self.request_type.clone(),
            timestamp: self.timestamp.clone(),
            endpoint_url: self.endpoint_url.clone(),
            status_code: self.status_code,
            responsetime: self.responsetime,
        }
    }
}
