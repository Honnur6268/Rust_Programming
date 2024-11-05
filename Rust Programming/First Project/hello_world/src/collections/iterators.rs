// Itrerators: Iterators are a way to loop over a collection of items, such as an array or a vector.
// An iterator is a trait that is used to implement the behavior of iterating over a collection of items.
// The Iterator trait provides a number of methods that can be used to iterate over a collection, such as map, filter, and fold.

// We can iterate over a collection using following ways
// 1. iter(): It borrows each element(immutable reference) of the collection through each iteration. Doesn't take ownership of the collection.
// 2. iter_mut(): It mutably borrows(reference) each element of the collection through each iteration. Doesn't take ownership of the collection.
// 3. into_iter(): It consumes the collection and returns an iterator that takes ownership of each element. It takes ownership of the collection.

// We have 2 types of Iterators:
// 1. Consuming adaptors: These methods consume the iterator and return the result of the iteration. For example, sum, collect, and for_each.
// 2. Iterator adaptors: These methods return a new iterator that is based on the original iterator. For example, map, filter, and enumerate.


pub fn iterator_demo(){
    let vector = vec![1, 2, 3, 4];
    println!("Original Vecto:\n{:?}", vector);
    let v1_iter = vector.iter();

    println!("\nIterating over the vector:");
    for val in v1_iter{
        println!("{}", val);
    }

    let mut vector2 = vec![1, 2, 3, 4];
    println!("\nOriginal Vector:\n{:?}", vector2);
    let v2_iter = vector2.iter_mut();

    // it will update the value of the vector2
    for val in v2_iter{
        *val += 10;
        println!("{}", val);
    }

    println!("\nAfter Iterating over the vector:\n{:?}", vector2);

    let vector3 = vec![1, 2, 3, 4];
    println!("\nOriginal Vector:\n{:?}", vector3);

    // it will consume the vector3 and take ownership of each element and return the iterator
    let v3_iter = vector3.into_iter();

    for val in v3_iter{
        println!("{}", val);
    }

    // println!("\nAfter Iterating over the vector:\n{:?}", vector3); // This will cause an error because vector3 is consumed by the into_iter method

    // Consuming adaptors:
    println!("\n------------ Consuming adaptors ------------");
    let vector4 = vec![1, 2, 3, 4];
    let v4_iter = vector4.iter();
    let sum: i32 = v4_iter.sum();
    println!("Sum of the vector: {}", sum);
    println!("After Consuming the vector:\n{:?}", vector4);
    // println!("{:?}",v4_iter)// This will cause an error because vector4 is consumed by the sum method

    // Iterator adaptors:
    println!("\n------------ Iterator adaptors ------------");
    let vector5 = vec![1, 2, 3, 4];
    println!("Original Vector:\n{:?}", vector5);
    let v5_iter = vector5.iter();
    let new_v5 = v5_iter.map(|x| *x * 10);
    println!("\nAfter applying map method on the vector:");
    for val in new_v5{
        println!("{}", val);
    }

}