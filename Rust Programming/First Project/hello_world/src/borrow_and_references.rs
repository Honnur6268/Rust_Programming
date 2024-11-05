// borrow and references rules
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

pub fn borrow_and_references() {
    // Immutable reference
    // println!("\n---------- Immutable reference ----------");
    // let s1 = String::from("hello");

    // Borrowing s1 by passing a reference to it
    // let len = calculate_length(&s1); // s1 is borrowed here. s1 address is passed to calculate_length() function

    // println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    println!("\n---------- Mutable reference ----------");
    let mut s2 = String::from("hello");

    // let s3 = &s2; // s2 is borrowed as reference here.
    // s3.push_str(" Rust Programmer"); // This will throw an error because s2 is borrowed as immutable reference
    // println!("{}", s3);

    // let s4 = &s2; // s2 is borrowed as reference here.
    // let s5 = &s2;

    // let s6 = &mut s2; // s2 is borrowed as mutable reference here. Must last until it’s used in the println!
    // s6.push_str(" Rust Programmer"); // This will work because s2 is borrowed as mutable reference
    // println!("{}", s6); // here s6 ig goes out of scope and s2 is no longer borrowed as mutable reference

    // let s7 = &mut s2; // because of above line again s2 is borrowed as mutable reference here. This works fine.
    // println!("{}", s7); // here s7 goes out of scope and s2 is no longer borrowed as mutable reference

    // Only one mutable reference is allowed at a time before it goes out of scope
    // let s8 = &mut s2; // s2 is borrowed as mutable reference here. This is still in scope. Must last until it’s used in the println!
    // let s9 = &mut s2; // Rust allows only one mutable reference to a variable at a time. This will throw an error.
    // println!("{}", s8);
    // println!("{}", s9);

    // we can have multiple immutable references to a variable at a time
    {
        println!("\nInside block");
        let s10 = &mut s2;
        s10.push_str(" Rustecean");
        println!("{}", s2);
    } // here s10 goes out of scope and s2 is no longer borrowed as mutable reference
    let s11 = &mut s2; // So we can borrow s2 as mutable reference here
    s11.push_str(" Programmer");
    println!("{}", s2);

    // update(&s2); // s2 is borrowed as mutable reference here. Will give an error because s2 is borrowed as mutable reference
    // update(&mut s2); // s2 is borrowed as mutable reference here.
    // println!("{}", s2);
}

// Function that takes a reference to a String
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn update(s: &mut String) {
    s.push_str(", world");
}
