use super::method::{Method, MethodError};
use super::query_string::QueryString;
// use std::fmt::Formatter;
use std::convert::TryFrom;
use std::error::Error;
use std::str::Utf8Error;
use std::str;

// use core::fmt::{ Display, Debug, Result as fmtResult };
use std::fmt::{Debug, Display, Formatter, Result as fmtResult};

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(buf: &'a [u8]) -> Result<Request<'a>, Self::Error>{
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or( ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or( ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or( ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }

        let method:Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find("?"){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        return Ok(Self{
            path: path,
            query_string: query_string,
            method: method,
        });
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "InvalidProtocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}