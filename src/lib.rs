use std::fmt::{self, Display};

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_responses
#[derive(Debug)]
pub enum HttpRequestCode {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
}
impl HttpRequestCode {
    pub fn to_tuple(&self) -> (usize, &'static str) {
        match self {
            HttpRequestCode::Ok => (200, "OK"),
            HttpRequestCode::BadRequest => (400, "Bad Request"),
            HttpRequestCode::Forbidden => (403, "Forbidden"),
            HttpRequestCode::NotFound => (404, "Not Found"),
        }
    }
}

#[derive(Debug)]
pub enum HttpError {
    HttpParseError(String, HttpRequestCode),
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

