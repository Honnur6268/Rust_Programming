/*!
* Ownership
   Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

   The Stack and the Heap: Memory and Allocation
       Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom would be more work.

       Heap memory is less organized: when you put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap, and is sometimes abbreviated as just allocating. Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the operating system must first find a big enough spot of unused memory and then perform bookkeeping to prepare for the next allocation.

   Ownership Rules:
       1. Each value in Rust has a variable that’s called its owner.
       2. There can only be one owner at a time.
       3. When the owner goes out of scope, the value will be dropped.
*/

pub fn ownership() {
    let str1 = String::from("hello");
    let str2 = str1;

    // println!("str1: {}", str1); // This will throw an error because str1 has been moved to str2

    println!("str2: {}", str2);

    let str3 = str2.clone(); // Still str2 is ownner, str3 is a clone of str2
    println!("After clone() str2: {}", str2);
    println!("After clone() str3: {}", str3);

    println!("\n\n---------- Ownership with functions ----------");
    let mut s1 = String::from("Hello");
    println!("s1: {}", s1);

    // take_ownership(s1); // here, s1 is moved to take_ownership() function
    // println!("s1: {}", s1); // This will throw an error because s1 has been moved to take_ownership() function

    // take_ownership(s1.clone()); // This will work because s1 is cloned and passed to take_ownership() function
    // println!("Inside ownership() s1: {}", s1); // This will print the value of s1 because s1 is cloned and passed to take_ownership() function

    s1 = takes_and_returns_ownership(s1); // here, s1 is moved to takes_and_returns_ownership() function and returned back to s1
    println!("Inside ownership() --> After takes_and_returns_ownership() s1: {}", s1); // This will print the value of s1
}

pub fn take_ownership(s: String) {
    println!("Inside take_ownership() s: {}", s);
}

pub fn takes_and_returns_ownership(s: String) -> String {
    println!("Inside takes_returns_ownership() s: {}", s);
    s
}
