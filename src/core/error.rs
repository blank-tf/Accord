/// The Accord error type.
pub enum Error {
    /// Indicates that the request was not successful.
    HTTP_CODE(i32),
    /// Indicates that no response came from curl.
    CURL_NO_RESPONSE,
    /// Indicates that a non-standard HTTP code was received.
    UNUSUAL_HTTP_CODE(i32),
    /// Indicates that a bad value was used as a parameter for a routine.
    BAD_PARAMETER,
    /// Indicates an internal failure when encoding or decoding JSON.
    BAD_JSON,
    /// Indicates an internal error in curl's easy interface.
    CURLE_INTERNAL,
    /// Indicates that globals were tried to be initialized more than once.
    GLOBAL_INIT,
}
