use std::net::TcpStream;

//https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_responses
#[derive(Debug)]
pub enum HttpErrorCode {
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
}

#[derive(Debug)]
pub enum HttpError {
    HttpParseError(String, HttpErrorCode),
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
                HttpErrorCode::BadRequest,
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
                HttpErrorCode::BadRequest,
            )),
        }
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
                HttpErrorCode::BadRequest,
            )),
        }
    }
}

