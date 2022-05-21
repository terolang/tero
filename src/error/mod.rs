/**
 * Defines different error types that exist in the language.
 */
pub enum ErrorType {
    SyntaxError,
}

/**
 * Error is the error type for the parser and code generator.
 * It contains a type and a message.
 * The type determines the kind of error.
 * The message is a human-readable description of the error.
 * 
 * TODO: Add a stack trace
 */
pub struct Error {
    pub code: ErrorType,
    pub message: String,
}

impl Error {
    pub fn new(code: ErrorType, message: String) -> Error {
        Error {
            code: code,
            message: message,
        }
    }
}
