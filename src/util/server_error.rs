use std::fmt;

#[derive(Debug, Clone)]
pub struct ServerError {
    code : String,
    message: String
}

impl ServerError {
    pub fn new(code: &str, message: &str) -> ServerError {
        ServerError {
            code: String::from(code),
            message: String::from(message)
        }
    }

    pub fn get_response_start(&self) -> String {
        let code = &self.code;
        let message = &self.message;
        return format!("HTTP/1.1 {code} {message}");
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = &self.code;
        let message = &self.message;
        write!(f, "{code}: {message}")
    }
}