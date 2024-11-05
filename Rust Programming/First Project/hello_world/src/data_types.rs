pub fn data_types() {
    // Scalar types: Represents a single value and includes:
    // 1. Integer types:
    //     - Signed: i8, i16, i32, i64, i128, isize
    //     - Unsigned: u8, u16, u32, u64, u128, usize
    // 2. Floating point types:
    //     - f32, f64
    // 3. Boolean type:
    //     - bool
    // 4. Character type:
    //     - char

    println!("---------------- Scalar types ---------------");

    // Integer types
    println!("--------- Integer types ---------");
    let int8: i8 = -128;
    let uint8: u8 = 255;
    let int32: i32 = -2147483648;
    let uint32: u32 = 4294967295;

    println!("int8: {}", int8);
    println!("uint8: {}", uint8);
    println!("int32: {}", int32);
    println!("uint32: {}\n", uint32);

    // Floating point types
    println!("--------- Floating point types ---------");
    let float32: f32 = 3.14;
    let float64: f64 = 2.718281828459045;
    println!("float32: {}", float32);
    println!("float64: {}\n", float64);

    // Boolean type
    println!("--------- Boolean type ---------");
    let is_rust_fun: bool = true;
    println!("is_rust_fun: {}", is_rust_fun);

    // Character type
    println!("\n--------- Character type ---------");
    let letter: char = 'R';
    let cat: char = '😻';
    println!("letter: {}", letter);
    println!("Cat: {}", cat);

    // String type
    println!("\n--------- String type ---------");
    let greeting: &str = "Hello, world!";
    let mut owned_string: String = String::from("Hello, Rust!");
    println!("greeting: {}", greeting);
    println!("owned_string: {}", owned_string);

    // Modifying the owned string
    owned_string.push_str(" Let's code!");
    println!("modified owned_string: {}", owned_string);

    println!("\n\n---------------- Compound types ---------------");
    //Compound types: Represents multiple values in one type and includes:
    // 1. Array type
    // 2. Tuple type

    // Array type: A collection of values of the same type and of a fixed size.
    println!("\n--------- Array type ---------");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    let a = [3; 5]; // This will create an array of 5 elements with the value 3
    println!("a: {:?}", a);

    // Tuple type: A tuple is a collection of values of different types
    // and of a fixed size. Once declared, they cannot grow or shrink in size.
    println!("\n--------- Tuple type ---------");
    let tuple: (i32, i32, f64, char) = (42, 86, 6.28, 'x');
    println!("tuple: {:?}", tuple);
    let (a, b, c, d) = tuple;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);

    // we can also access the tuple values using the dot operator or period(.).
    let x = tuple.0;
    println!("The value of x is: {}", x);

    // unit tuple  // A tuple with no elements is called a unit tuple.
    let unit_tuple = ();
    println!("unit_tuple: {:?}", unit_tuple);
}
