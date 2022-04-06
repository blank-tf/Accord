use crate::core::user_agent::HTTPMethod::{
    HTTPDelete, HTTPGet, HTTPInvalid, HTTPMimePost, HTTPPatch, HTTPPost, HTTPPut,
};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

/// See [this webpage](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) for more info.
enum HTTPStatusCode {
    HTTPOk = 200,
    HTTPCreated = 201,
    HTTPNoContent = 204,
    HTTPNotModified = 304,
    HTTPBadRequest = 400,
    HTTPUnauthorized = 401,
    HTTPForbidden = 403,
    HTTPNotFound = 404,
    HTTPMethodNotAllowed = 405,
    HTTPUnprocessableEntity = 422,
    HTTPTooManyRequests = 429,
    HTTPGatewayUnavailable = 502,
    Other(i32),
}

/// HTTP Methods.
enum HTTPMethod {
    HTTPInvalid,
    HTTPDelete,
    HTTPGet,
    HTTPPost,
    HTTPMimePost,
    HTTPPatch,
    HTTPPut,
}

struct ParseHTTPMethodError;

impl FromStr for HTTPMethod {
    type Err = ParseHTTPMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(HTTPDelete),
            "GET" => Ok(HTTPGet),
            "POST" => Ok(HTTPPost),
            "MIMEPOST" => Ok(HTTPMimePost),
            "PATCH" => Ok(HTTPPatch),
            "PUT" => Ok(HTTPPut),
            _ => Err(ParseHTTPMethodError),
        }
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            HTTPMethod::HttpInvalid => "INVALID_HTTP_METHOD",
            HTTPMethod::HttpDelete => "DELETE",
            HTTPMethod::HttpGet => "GET",
            HTTPMethod::HttpPost => "POST",
            HTTPMethod::HttpMimePost => "MIMEPOST",
            HTTPMethod::HttpPatch => "PATCH",
            HTTPMethod::HttpPut => "PUT",
        };

        write!(f, "{}", result)
    }
}

impl Display for HTTPStatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            HTTPStatusCode::HTTPOk => "OK",
            HTTPStatusCode::HTTPCreated => "CREATED",
            HTTPStatusCode::HTTPNoContent => "NO_CONTENT",
            HTTPStatusCode::HTTPNotModified => "NOT_MODIFIED",
            HTTPStatusCode::HTTPBadRequest => "BAD_REQUEST",
            HTTPStatusCode::HTTPUnauthorized => "UNAUTHORIZED",
            HTTPStatusCode::HTTPForbidden => "FORBIDDEN",
            HTTPStatusCode::HTTPNotFound => "NOT_FOUND",
            HTTPStatusCode::HTTPMethodNotAllowed => "METHOD_NOT_ALLOWED",
            HTTPStatusCode::HTTPUnprocessableEntity => "UNPROCESSABLE_ENTITY",
            HTTPStatusCode::HTTPTooManyRequests => "TOO_MANY_REQUESTS",
            HTTPStatusCode::HTTPGatewayUnavailable => "GATEWAY_UNAVAILABLE",
            HTTPStatusCode::Other(code) => match code {
                100..200 => "1xx_INFO",
                200..300 => "2xx_SUCCESS",
                300..400 => "3xx_REDIRECTING",
                400..500 => "4xx_CLIENT_ERROR",
                500..600 => "5xx_SERVER_ERROR",
                _ => "UNUSUAL_HTTP_CODE",
            },
        };
        write!(f, "{}", result)
    }
}

impl HTTPStatusCode {
    fn reason(&self) -> String {
        match &self {
            HTTPStatusCode::HTTPOk => String::from("The request was completed successfully."),
            HTTPStatusCode::HTTPCreated => String::from("The entity was created successfully."),
            HTTPStatusCode::HTTPNoContent => String::from("The request completed successfully but returned no content."),
            HTTPStatusCode::HTTPNotModified => String::from("The entity was not modified (no action was taken)."),
            HTTPStatusCode::HTTPBadRequest => String::from("The request was improperly formatted, or the server couldn't understand it."),
            HTTPStatusCode::HTTPUnauthorized => String::from("The authorization header was missing or invalid."),
            HTTPStatusCode::HTTPForbidden => String::from("The authorization token you passed did not have permission to the resource."),
            HTTPStatusCode::HTTPNotFound => String::from("The resource at the location specified doesn't exist."),
            HTTPStatusCode::HTTPMethodNotAllowed => String::from("The HTTP method used is not valid for the location specified."),
            HTTPStatusCode::HTTPUnprocessableEntity => String::from("The entity was unprocessable."),
            HTTPStatusCode::HTTPTooManyRequests => String::from("You got rate-limited."),
            HTTPStatusCode::HTTPGatewayUnavailable => String::from("There was not a gateway available to process your request. Wait a bit and retry."),
            HTTPStatusCode::Other(code) => {
                match code {
                    0 => String::from("Curl couldn't fetch a HTTP response."),
                    100..200 => String::from("The request was received, understood and accepted. The client must wait for a final response."),
                    200..300 => String::from("The action request was received, understood, and accepted."),
                    300..400 => String::from("Client requires taking additional action to complete the request."),
                    400..500 => String::from("Client side error, request couldn't be processed."),
                    500..600 => String::from("The server had an error processing your request."),
                    _ => String::from("Unusual HTTP method.")
                }
            }
        }
    }
}

struct UAConnection {
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
    idle: queue,
    busy: queue,
    total: i32,
}

struct UserAgent {
    ua_conn_queue: Box<conq>,
    base_url: String,
    // logconf

    // TODO: https://github.com/Cogmasters/concord/blob/master/core/user-agent.c#L30 implement this???
}
