/// The Accord error type.
pub enum Error {
    /// Indicates that the request was not successful.
    HttpCode(i32),
    /// Indicates that no response came from curl.
    CurlNoResponse,
    /// Indicates that a non-standard HTTP code was received.
    UnusualHttpCode(i32),
    /// Indicates that a bad value was used as a parameter for a routine.
    BadParameter,
    /// Indicates an internal failure when encoding or decoding JSON.
    BadJson,
    /// Indicates an internal error in curl's easy interface.
    CurlEInternal,
    /// Indicates that globals were tried to be initialized more than once.
    GlobalInit,
}
