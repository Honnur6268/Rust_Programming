// use chrono::{DateTime, Local, Utc};
// use enums::Shape;
// // use structs::Rectangle;
// use options::find_first_a;
// use rand::{thread_rng, Rng};

pub mod control_flows;
pub mod data_types;
pub mod variables;
// mod functions;
pub mod borrow_and_references;
pub mod collections;
pub mod enums;
pub mod error_handling;
pub mod options;
pub mod ownership;
pub mod stack_heap;
pub mod structs;
pub mod rust_features;
pub mod threads;
pub mod threads_message_passing;

fn main() {
    // println!("Hello, rust programmer!");
    // variables::variable();
    // data_types::data_types();
    // control_flows::control_flow();

    // functions::sum
    /*
    functions::helloworld();

    let mut num1 = 10;
    let mut num2 = 20;

    let add = functions::sum(num1, num2);
    println!("The sum of {num1} and {num2} is: {}", add);

    num2 = 40;
    num1 = 30;
    println!("The sum of {num1} and {num2} is: {}", functions::sum(num1, num2));
    */

    // stack_heap::stack_fn();
    // stack_heap::heap_fn();
    // stack_heap::update_string();

    // ownership
    // ownership::ownership();

    // Scope
    // let s = "hello";
    // println!("Outside block: s: {}", s);
    // {
    //     let s1 = "world";
    //     println!("Inside block: s: {}", s);
    //     println!("Inside block: s1: {}", s1);
    // }
    // println!("Outside block: {}", s1);

    // borrow_and_references
    // borrow_and_references::borrow_and_references();

    // structs
    // let user = structs::get_user();
    // println!("{:#?}", user);

    // let rectangle = structs::Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("\n{rectangle:#?}");

    // println!(
    //     "\nThe area of the rectangle is: {}",
    //     structs::area(&rectangle)
    // );

    // let circle = structs::Circle{radius: 10.0};
    // println!("\nThe area of the circle is: {}", circle.area_of_circle());

    // let mut c = circle;
    // c.radius = 20.0;
    // println!("\nThe area of the circle is: {}", c.area_of_circle());
    // println!("\nThe area of the circle is: {}", circle.area()); // This will cause an error because circle has been moved to c (ownership has been moved to c)

    // Enums and Pattern Matching
    // println!("\n\n---------- Enums and Pattern Matching ----------");
    // let circle = Shape::Circle(10.0);
    // let square = Shape::Square(10.0);
    // let Rectangle = Shape::Rectangle(10.0, 20.0);

    // println!(
    //     "The area of the circle is: {}",
    //     enums::calculate_area(circle)
    // );
    // println!(
    //     "The area of the square is: {}",
    //     enums::calculate_area(Shape::Square(10.0))
    // );
    // println!(
    //     "The area of the rectangle is: {}",
    //     enums::calculate_area(Shape::Rectangle(10.0, 20.0))
    // );

    // Error Handling
    // println!("\n\n---------- Error Handling ----------");
    // let res = error_handling::read_file("example.txt".to_string());
    // println!("The content of the file is: {:?}", res);

    // let res_2 = error_handling::read_file_2("example.txt".to_string());
    // println!("\n{:?}", res_2);

    // let res_3 = error_handling::read_file_3("example.txt".to_string());
    // println!("\n{:?}", res_3);

    // Option Enum:
    // println!("\n\n---------- Option Enum ----------");
    // let five = Some(5);
    // let six = enums::plus_one(five);
    // println!("\nThe value of six is: {:?}", six);
    // let none = enums::plus_one(None);
    // println!("\nThe value of none is: {:?}", none);

    // // Find First 'a' or 'A' in a String
    // let str = String::from("Honnur Ali");
    // match find_first_a(str) {
    //     Some(index) => println!("\nFound the first 'a' or 'A' at index: {}", index),
    //     None => println!("\nThe letter 'a' or 'A' is not found in the string."),
    // };

    // Random Number Generator
    // let mut rng = thread_rng();
    // let random_number: u32 = rng.gen();
    // println!("\nRandom number: {}", random_number);

    // let random_number_range: u32 = rng.gen_range(1..=100);
    // println!(
    //     "\nRandom number in the range of 1 to 100: {}",
    //     random_number_range
    // );

    // Date and Time
    // let local_date_time: DateTime<Local> = Local::now();
    // println!("\nLocal Date and Time: {}", local_date_time);

    // let utc = Utc::now();
    // println!("\nUTC Date and Time: {}", utc);

    // Collections
    // println!("\n\n---------- Collections ----------");
    // Vector
    // println!("\n---------- Vector ----------");
    // collections::vector::vector_demo();

    // Hashmap
    // println!("\n---------- Hashmap ----------");
    // collections::hashmap::hashmap_demo();

    // Iterator
    // println!("\n---------- Iterator ----------");
    // collections::iterators::iterator_demo();

    // Strings
    // println!("\n---------- Strings ----------");
    // collections::strings::string_demo();

    // Generics
    // println!("\n---------- Generics ----------");
    // collections::generics::generics_demo();

    // Traits
    // println!("\n---------- Traits ----------");
    // rust_features::traits::traits_demo();

    // Lifetimes
    // println!("\n---------- Lifetimes ----------");
    // rust_features::lifetimes::lifetime_demo();

    // Threads
    // println!("\n---------- Threads ----------");
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(std::time::Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();

    // for i in 1..10 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(std::time::Duration::from_millis(1));
    // }
    // threads::thread_demo();

    // Message Passing to Transfer data between threads
    println!("\n---------- Message Passing to Transfer data between threads ----------");
    threads_message_passing::msg_passing_demo();
    
    
}
