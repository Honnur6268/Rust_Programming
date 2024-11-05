// Vector: a dynamically sized array. Vectors are used to store multiple values of the same type in a single data structure.
// Vector is a growable array. It is a heap-allocated array that can grow or shrink in size.

use std::vec;

pub fn vector_demo() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);

    println!("{:?}", vector);

    println!("The first element of the vector is: {}", vector[3]);

    // We can create vector using macros
    let vector_2 = vec![1, 2, 3, 4, 5];
    println!("\n{:?}", vector_2);

    // we can get element from the vector using two ways
    // 1. using index
    println!("The first element of the vector is: {}", vector_2[0]);
    // 2. using get method
    let ind = 5;
    let second_element = vector_2.get(ind);
    match second_element{
        Some(value) => println!("The second element of the vector is: {}", value),
        None => println!("The element is not found at index {}", ind),
    }

    // We can also create vector using the from method
    let mut vector_3 = Vec::from([1, 2, 3, 4, 5]);
    println!("\n{:?}", vector_3);
    println!("The first element of the vector is: {}", vector_3[0]);
    println!("The length of the vector is: {}", vector_3.len());

    // we can pop  the element from the vector
    let poped = vector_3.pop();
    println!("\nThe poped element is: {:?}", poped.unwrap());
    println!("After Poping:\n{:?}", vector_3);

    // we can remove the element from the vector
    let index = 2;
    let removed_element = vector_3.remove(index);
    println!(
        "\nThe removed element at index {} is: {:?}",
        index, removed_element
    );
    println!("After Removing:\n{:?}", vector_3);

    // println!("\nEven Number\n {:?}",even_number_filter(vector_2));
    // println!("{:?}",vector_2); // This will cause an error because vector_2's ownership has been moved to the even_number_filter function

    let result = even_number_filter(&vector_2);
    println!("\nEven Numbers: {:?}", result);
    println!("{:?}", vector_2);

    vector_3.push(4);
    println!("\nBefore Removing Odd Numbers: {:?}", vector_3);
    even_number_filter_mut(&mut vector_3);
    println!("\nEven Numbers: {:?}", vector_3);
}

fn even_number_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut even_numbers = Vec::new();
    for number in vec {
        if number % 2 == 0 {
            even_numbers.push(*number);
        }
    }
    return even_numbers;
}

fn even_number_filter_mut(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
