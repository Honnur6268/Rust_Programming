// Option Enum: 
        // Option<T> is an enum defined by the standard library. It is used when absence is a possibility. It is used when a value could be something or it could be nothing. 
        // The Option<T> enum has two variants:
            // 1. Some(T): Represents a value of type T
            // 2. None: Represents the absence of a value
        // The Option<T> enum is used in the context of error handling, where a function that might fail returns an Option<T> value instead of panicking or returning a Result<T, E> value. 
        // The Option<T> enum is also used in the context of nullability, where a value might be null or it might be a valid value.
        // The Option<T> enum is used in the context of initial values, where a value might be uninitialized or it might be a valid value.
        // The Option<T> enum is used in the context of optional values, where a value might be present or it might be absent.

pub fn find_first_a(s: String) -> Option<usize>{
    for (index, char) in s.chars().enumerate(){
        if char == 'a' || char == 'A'{
            return Some(index);
        }
    }
    return None;
}