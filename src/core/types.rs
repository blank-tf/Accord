/// The types for the Accord project. In some cases tuple structs are used instead of the `type` keyword to reduce data format mistakes

/// Unix time in milliseconds.
/// Commonly used for fields that may store timestamps.
struct UnixMs(u64);

/// Snowflake datatype.
/// Used in APIs such as Twitter and Discord for their unique IDs.
struct Snowflake(u64);

/// Bitmask primitive
/// Used for fields that may store this type, or perform bitwise operations on.
type Bitmask = u64;

// probably don't need json_char
// leaving a note here just in-case.
