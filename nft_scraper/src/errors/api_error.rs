use super::{
    GenericError, HeaderToStrError, HttpError, HyperError, JsonError, StringFromUtf8Error,
};
use std::{error, fmt};

pub(crate) type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum ApiError {
    Http(HttpError),
    Hyper(HyperError),
    StringFromUtf8(StringFromUtf8Error),
    HeaderToStr(HeaderToStrError),
    Json(JsonError),
    BadRequest(String),
    Fatal(String),
    Other(GenericError),
}

impl ApiError {
    pub fn bad_request<'a>(cause: &'a str) -> Box<dyn FnOnce() -> Self + 'a> {
        Box::new(move || Self::BadRequest(cause.to_string()))
    }

    pub fn bad_request_err<'a, T>(cause: &'a str) -> ApiResult<T> {
        Err(Self::BadRequest(cause.to_string()))
    }

    pub fn fatal<'a>(cause: &'a str) -> Box<dyn FnOnce() -> Self + 'a> {
        Box::new(move || Self::Fatal(cause.to_string()))
    }
}

impl error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Http(ref err) => write!(f, "Http error: {}", err),
            Self::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            Self::StringFromUtf8(ref err) => {
                write!(f, "String from utf8 error: {:?}", err)
            }
            Self::HeaderToStr(ref err) => write!(f, "Header to string error: {:?}", err),
            Self::Json(ref err) => write!(f, "Json error: {:?}", err),
            Self::BadRequest(ref cause) => write!(f, "Bad request error: {}", cause),
            Self::Fatal(ref cause) => write!(f, "Fatal error: {}", cause),
            Self::Other(ref err) => write!(f, "Other error: {:?}", err),
        }
    }
}
