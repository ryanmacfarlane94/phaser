use crate::util::server_error::ServerError;

pub struct Request {
    start_line: String,
    headers:String,
    body:String
}

impl Request {
    pub fn new(request: Vec<String>) -> Result<Request, ServerError> {
        let start_line = match request.get(0) {
            Some(start_line) => start_line,
            None => return Err(ServerError::new("400","Bad Request"))
        };

        Ok(Request {
            start_line: start_line.to_string(),
            headers: String::new(),
            body: String::new(),
        })
    }
}
