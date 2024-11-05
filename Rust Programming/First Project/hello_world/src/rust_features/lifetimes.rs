use std::fmt::Display;
// Lifetime: Lifetime is a concept in Rust that helps in preventing dangling references.
// Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid/
// The Rust compiler uses lifetimes to ensure that all borrows are valid.
// Lifetimes are a way of annotating the relationships between the lifetimes of various references in our code.

// without Lifetime
/* fn longest(str1: String, str2: String) -> String { // works fine
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
    */

// fn longest(str1: &str, str2: &str) -> &str { // Gives error: missing lifetime specifier
//     // wprks fine
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// to fix the error, we need to add lifetime annotation
// Lifetime Annotation: Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
// Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
// Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. 
// Most people use the name 'a for the first lifetime annotation. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str { // Lifetime annotation
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// Structs with Lifetimes
struct User<'b> {
    name: &'b str
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
pub fn longest_with_an_announcement<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

pub fn lifetime_demo() {
    /* let str1 = String::from("Welcome");
    let str2 = String::from("Rust Programming");
    let result = longest(str1, str2);
    println!("The longest string is: {}", result);*/

    // let str1 = String::from("Welcome");
    // let str2 = String::from("Rust Programming");
    // let result = longest(&str1, &str2);
    // println!("The longest string is: {}", result);

    let result;
    let str1 = String::from("Welcome");
    {
        let str2 = String::from("Rust Programming");
        result = longest(&str1, &str2); // Gives error: borrowed value does not live long enough. The str2 variable goes out of scope at the end of the block, but we’re trying to return a reference to it.
        println!("The longest string is: {}", result); // It will work because result is assigned a value before the it goes out of scope.
    }
    // println!("The longest string is: {}", result); // It will not work. Try to access  by commenting line 55

    // Structs with Lifetimes
    println!("\n---------- Structs with Lifetimes ----------");
    // let user = User{name: "Honnur Ali"};
    // println!("User name: {}", user.name);
    let user;
    {
        let name = String::from("Honnur Ali");
        user = User{name: &name}; // If we access user outside of this scope then Gives error: borrowed value does not live long enough. The name variable goes out of scope at the end of the block, but we’re trying to return a reference to it.
        println!("User name: {}", user.name); // Works fine
    }
    // println!("User name: {}", user.name); // user goes out of scope here

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    println!("\n---------- Generic Type Parameters, Trait Bounds, and Lifetimes Together ----------");
    let str1 = String::from("Welcome");
    let str2 = String::from("Rust Programming");
    let ann = "Printing Data......";
    let result = longest_with_an_announcement(&str1, &str2, ann);
    println!("The longest string is: {}", result);

}
