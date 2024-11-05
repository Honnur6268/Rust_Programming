pub fn variable() {
    // Immutable variable
    let x = 18;
    println!("The value of x is: {x}");

    // x = 128; // This will throw an error because x is immutable

    // Mutable variable
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // Variable shadowing
    let z = 15;
    println!("The value of z is: {}", z);
    let z = z + 5; 
    {
        let z = z * 2;
        println!("The inner shadowed value of z is: {}", z);
    }
    println!("The outer shadowed value of z is: {}", z);

    // Constants
    const MAX_POINTS: u32 = 10000;
    println!("The maximum points are: {}", MAX_POINTS);
}