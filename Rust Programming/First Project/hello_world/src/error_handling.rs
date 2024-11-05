use std::{fs, io::Error};

// Error Handling: Error handling is the process of responding to and recovering from error conditions in your program. Error handling in Rust is done using the Result<T, E> enum. The Result<T, E> enum is defined by the standard library and is used to represent the result of an operation that can fail. 
// The Result<T, E> enum has two variants:
//    1. Ok(T): Represents a successful result with a value of type T
//    2. Err(E): Represents a failed result with a value of type E
// The Result<T, E> enum is used in the context of error handling, where a function that might fail returns a Result<T, E> value instead of panicking or returning an Option<T> value.
// The Result<T, E> enum is also used in the context of error propagation, where a function that might fail returns a Result<T, E> value that is propagated up the call stack.
// The Result<T, E> enum is used in the context of error recovery, where a function that might fail returns a Result<T, E> value that is handled by the caller.

pub fn read_file(file_content: String) -> Result<String, Error> {
    let result = fs::read_to_string(file_content);
    match result {
        Ok(content) => Ok(content),
        Err(error) => Err(error),
    }
}

pub fn read_file_2(file_content: String) -> Result<String, String> {
    let result = fs::read_to_string(file_content);
    match result {
        Ok(content) => return Ok(content),
        Err(_error_) => return Err("Error reading the file...".to_string()),
    }
}

pub fn read_file_3(file_content: String) -> Result<String, String> {
    let result = fs::read_to_string(file_content);
    if let Ok(content) = result {
        return Ok(content);
    } else {
        return Err("Error reading the file...".to_string());
    }
}
