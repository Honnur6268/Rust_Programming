pub fn control_flow() {
    println!("---------------- Control flows ---------------");

    // if-else control flow
    println!("\n--------- If-else control flow ---------");
    let number = -1;
    if number < 10 && number > 5 {
        println!("The number is less than 10 and greater than 5");
    } else if number < 5 && number > 0 {
        println!("The number is less than 5 and greater than 0");
    } else {
        println!("The number is not a positive number");
    }

    println!("\n------------ Repetition control flow ------------");
    // Repetition control flow: loop, while, for
    // loop control flow: loop keyword is used to create an infinite loop. And we can use break to exit the loop.
    println!("\n--------- Loop control flow ---------");
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
        println!("loop {}",count)
    }
    println!("Loop executed {} times", count);

    // while control flow
    println!("\n--------- While control flow ---------");
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // for control flow
    println!("\n--------- For control flow ---------");
    let b = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    for element in b.iter() {
        println!("The value is: {}", element);
    }

    for element in b{
        println!("\nThe value is: {}", element);
    }

    println!();

    for element in b.iter().rev(){
        println!("The value is: {}", element);
    }

    // match control flow
    println!("\n--------- Match control flow ---------");
    let number = 2;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 => println!("Four!"),
        5 => println!("Five!"),
        _ => println!("Something else!"),
    }
}
