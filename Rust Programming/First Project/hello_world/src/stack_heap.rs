pub fn stack_fn(){
    let x = 10;
    let y = 20;
    let z = x + y;
    println!("The sum of {x} and {y} is: {z}");
}

pub fn heap_fn(){
    let x = String::from("Hello");
    let y = String::from("World");
    let z = format!("{}, {}", x, y);
    println!("{}", z);
}

pub fn update_string(){
    let mut x = String::from("Hello, Welcome To");
    println!("Before updating the string");
    println!("Capacity: {} length: {} pointer: {:p}", x.capacity(), x.len(), x.as_ptr());
    // println!("{}", x);

    x.push_str(" Rust Programming Language");
    println!("\nAfter updating the string");
    println!("Capacity: {} length: {} pointer: {:p}", x.capacity(), x.len(), x.as_ptr());
    // println!("{}", x);
}