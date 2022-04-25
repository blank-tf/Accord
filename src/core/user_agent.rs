use std::sync::Mutex;
use crate::core;
use curl::easy as Curl;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// See [this webpage](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) for more info.
enum HTTPStatusCode {
    Ok,
    Created,
    NoContent,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    UnprocessableEntity,
    TooManyRequests,
    GatewayUnavailable,
    Other(i32),
}

/// HTTP Methods.
enum HTTPMethod {
    Invalid,
    Delete,
    Get,
    Post,
    MimePost,
    Patch,
    Put,
}

struct ParseHTTPMethodError;

impl FromStr for HTTPMethod {
    type Err = ParseHTTPMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(HTTPMethod::Delete),
            "GET" => Ok(HTTPMethod::Get),
            "POST" => Ok(HTTPMethod::Post),
            "MIMEPOST" => Ok(HTTPMethod::MimePost),
            "PATCH" => Ok(HTTPMethod::Patch),
            "PUT" => Ok(HTTPMethod::Put),
            _ => Err(ParseHTTPMethodError),
        }
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            HTTPMethod::Invalid => "INVALID_HTTP_METHOD",
            HTTPMethod::Delete => "DELETE",
            HTTPMethod::Get => "GET",
            HTTPMethod::Post => "POST",
            HTTPMethod::MimePost => "MIMEPOST",
            HTTPMethod::Patch => "PATCH",
            HTTPMethod::Put => "PUT",
        };

        write!(f, "{}", result)
    }
}

impl Display for HTTPStatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            HTTPStatusCode::Ok => "OK",
            HTTPStatusCode::Created => "CREATED",
            HTTPStatusCode::NoContent => "NO_CONTENT",
            HTTPStatusCode::NotModified => "NOT_MODIFIED",
            HTTPStatusCode::BadRequest => "BAD_REQUEST",
            HTTPStatusCode::Unauthorized => "UNAUTHORIZED",
            HTTPStatusCode::Forbidden => "FORBIDDEN",
            HTTPStatusCode::NotFound => "NOT_FOUND",
            HTTPStatusCode::MethodNotAllowed => "METHOD_NOT_99LOWED",
            HTTPStatusCode::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            HTTPStatusCode::TooManyRequests => "TOO_MANY_REQUESTS",
            HTTPStatusCode::GatewayUnavailable => "GATEWAY_UNAVAILABLE",
            HTTPStatusCode::Other(code) => match *code {
                100..=199 => "1xx_INFO",
                200..=299 => "2xx_SUCCESS",
                300..=399 => "3xx_REDIRECTING",
                400..=499 => "4xx_CLIENT_ERROR",
                500..=599 => "5xx_SERVER_ERROR",
                _ => "UNUSUAL_HTTP_CODE",
            },
        };
        write!(f, "{}", result)
    }
}

impl HTTPStatusCode {
    fn reason(&self) -> String {
        match &self {
            HTTPStatusCode::Ok => String::from("The request was completed successfully."),
            HTTPStatusCode::Created => String::from("The entity was created successfully."),
            HTTPStatusCode::NoContent => String::from("The request completed successfully but returned no content."),
            HTTPStatusCode::NotModified => String::from("The entity was not modified (no action was taken)."),
            HTTPStatusCode::BadRequest => String::from("The request was improperly formatted, or the server couldn't understand it."),
            HTTPStatusCode::Unauthorized => String::from("The authorization header was missing or invalid."),
            HTTPStatusCode::Forbidden => String::from("The authorization token you passed did not have permission to the resource."),
            HTTPStatusCode::NotFound => String::from("The resource at the location specified doesn't exist."),
            HTTPStatusCode::MethodNotAllowed => String::from("The HTTP method used is not valid for the location specified."),
            HTTPStatusCode::UnprocessableEntity => String::from("The entity was unprocessable."),
            HTTPStatusCode::TooManyRequests => String::from("You got rate-limited."),
            HTTPStatusCode::GatewayUnavailable => String::from("There was not a gateway available to process your request. Wait a bit and retry."),
            HTTPStatusCode::Other(code) => {
                match code {
                    0 => String::from("Curl couldn't fetch a HTTP response."),
                    100..=199 => String::from("The request was received, understood and accepted. The client must wait for a final response."),
                    200..=299 => String::from("The action request was received, understood, and accepted."),
                    300..=399 => String::from("Client requires taking additional action to complete the request."),
                    400..=499 => String::from("Client side error, request couldn't be processed."),
                    500..=599 => String::from("The server had an error processing your request."),
                    _ => String::from("Unusual HTTP method.")
                }
            }
        }
    }
}

struct UAInfo {
    //loginfo
    // NOT RUST CORE, ACCORD CORE!
    code: Option<core::Error>,
    http_code: HTTPStatusCode,
    // header: UAResponseHeader,
    // body: UAResponseBody,
}

struct UAConnection<> 
{
    user_agent: Box<UserAgent>,
    easy_handle: Box<Curl::Easy>,
    info: UAInfo,
    url: String,
    // TODO: header: &curl_slist
    // TODO: https://github.com/Cogmasters/concord/blob/master/core/user-agent.c#L62 implement this???
    // TODO: errbuf: String ?

    // TODO: entry: queue
}
// this needs to be mutex'd
struct UAConnectionQueue {
    //idle: queue,
    //busy: queue,
    total: i32,
}

struct UserAgent {
    ua_conn_queue: Mutex<Box<UAConnectionQueue>>,
    base_url: String,
    // logconf

    // TODO: https://github.com/Cogmasters/concord/blob/master/core/user-agent.c#L30 implement this???
}
