use std::{
    collections::HashMap,
    fmt::{self, Display},
};

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_responses
#[derive(Debug)]
pub enum HttpRequestCode {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
    InternalServerError,
}
impl HttpRequestCode {
    pub fn to_tuple(&self) -> (usize, &'static str) {
        match self {
            HttpRequestCode::Ok => (200, "OK"),
            HttpRequestCode::BadRequest => (400, "Bad Request"),
            HttpRequestCode::Forbidden => (403, "Forbidden"),
            HttpRequestCode::NotFound => (404, "Not Found"),
            HttpRequestCode::InternalServerError => (500, "Internal Server Error"),
        }
    }
}
impl Display for HttpRequestCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (_, description) = self.to_tuple();
        write!(f, "{}", description)
    }
}

#[derive(Debug)]
pub enum HttpError {
    HttpParseError(String, HttpRequestCode),
}
impl Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::HttpParseError(msg, code) => {
                write!(f, "HTTP Parse Error ({}): {}", code, msg)
            }
        }
    }
}
impl From<std::io::Error> for HttpError {
    fn from(err: std::io::Error) -> Self {
        HttpError::HttpParseError(err.to_string(), HttpRequestCode::BadRequest)
    }
}

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    TRACE,
    CONNECT,
    DELETE,
    HEAD,
    OPTIONS,
}
impl TryFrom<String> for HttpMethod {
    type Error = HttpError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(HttpError::HttpParseError(
                format!("Invalid HTTP method: {}", s),
                HttpRequestCode::BadRequest,
            )),
        }
    }
}

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum HttpVersion {
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2_0,
    HTTP_3_0,
}
impl TryFrom<String> for HttpVersion {
    type Error = HttpError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "HTTP/1.0" => Ok(Self::HTTP_1_0),
            "HTTP/1.1" => Ok(Self::HTTP_1_1),
            "HTTP/2.0" => Ok(Self::HTTP_2_0),
            "HTTP/3.0" => Ok(Self::HTTP_3_0),
            _ => Err(HttpError::HttpParseError(
                format!("Invalid HTTP version: {}", s),
                HttpRequestCode::BadRequest,
            )),
        }
    }
}
impl Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HTTP_1_0 => "HTTP/1.0",
                Self::HTTP_1_1 => "HTTP/1.1",
                Self::HTTP_2_0 => "HTTP/2.0",
                Self::HTTP_3_0 => "HTTP/3.0",
            }
        )
    }
}

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#start_line
#[derive(Debug)]
pub struct StartLine {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
}
impl StartLine {
    pub fn new(start_line: &String) -> Result<Self, HttpError> {
        let mut parts = start_line.split_whitespace();

        match (parts.next(), parts.next(), parts.next()) {
            (Some(method), Some(path), Some(version)) => Ok(Self {
                method: HttpMethod::try_from(method.to_string())?,
                path: path.to_string(),
                version: HttpVersion::try_from(version.to_string())?,
            }),
            _ => Err(HttpError::HttpParseError(
                start_line.to_string(),
                HttpRequestCode::BadRequest,
            )),
        }
    }
}

//https://developer.mozilla.org/en-US/docs/Glossary/Representation_header
#[derive(Debug)]
pub struct RepresentationHeader {
    pub content_type: String,
    pub content_length: usize,
}
impl RepresentationHeader {
    pub fn new(content_type: &str, content_length: usize) -> RepresentationHeader {
        RepresentationHeader {
            content_type: content_type.to_string(),
            content_length,
        }
    }
}
impl fmt::Display for RepresentationHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Content-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.content_type, self.content_length
        )
    }
}

//https://developer.mozilla.org/en-US/docs/Glossary/Request_header
#[derive(Debug)]
pub struct RequestHeader {
    pub headers: HashMap<String, String>,
}
impl RequestHeader {
    pub fn from_http_request(http_request: &[String]) -> RequestHeader {
        let mut headers = HashMap::new();
        // Skip the start line
        for line in http_request.iter().skip(1) {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                headers.insert(key, value);
            }
        }
        RequestHeader { headers }
    }
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

#[derive(Debug)]
pub struct RequestBody {
    pub content: String,
}
impl RequestBody {
    pub fn new(content: String) -> RequestBody {
        RequestBody { content }
    }
}

