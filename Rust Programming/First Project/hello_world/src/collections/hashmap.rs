// Hashmap: It is a collection of key-value pairs. The key is used to access the value. The key and value can be of any type. The key must be unique, but the values can be duplicated.
// It is a hash table implementation of the map data structure. It is a data structure that stores key-value pairs in a way that allows for fast retrieval of the value associated with a given key.

use std::collections::HashMap;

pub fn hashmap_demo(){
    let mut map = HashMap::new();
    map.insert("Honnur", 26);
    map.insert("raju", 22);

    println!("{:?}", map);

    // we can create hashmap using the from method
    let map_2 = HashMap::from([(1, "one"), (2, "two"), (3, "three")]);
    println!("\n{:?}", map_2);

    // we can get the value from the hashmap using the key
    let key = "Honnuqdqr";
    let value = map.get(key);
    match value{
        Some(val) => println!("\nThe age of {} is: {}", key, val),
        None => println!("The key {} is not found in the db", key),
    }

    // we can remove the value from the hashmap using the key
    let key = "Honnur";
    let removed_value = map.remove(key);
    match removed_value{
        Some(val) => println!("\nThe removed key {} and value is: {}", key, val),
        None => println!("The key {} is not found in the db", key),
    }

    println!("\nAfter Removing:\n{:?}", map);

    // we can iterate over the hashmap
    let mut map_2 = HashMap::new();
    map_2.insert(101, "Honnur");
    map_2.insert(102, "Raju");
    map_2.insert(103, "Ali");

    println!("\n{:?}", map_2);
    println!("");
    for (key, value) in &map_2{
        println!("The key is: {} and the value is: {}", key, value);
    }

    // we can check if the key is present in the hashmap
    let key = 101;
    let is_key_present = map_2.contains_key(&key);
    match is_key_present{
        true => println!("\nThe key {} is present in the db and the value is {}", key, map_2.get(&key).unwrap()),
        false => println!("\nThe key {} is not present in the db", key),
    }

    // we can check if the value is present in the hashmap
    let value = "Honnur";
    let is_value_present = map_2.values().any(|val| val == &value);
    match is_value_present{
        true => println!("\nThe value {} is present in the db", value),
        false => println!("\nThe value {} is not present in the db", value),
    }

    let vector_tuple = vec![(101, String::from("Honnur")), (102, String::from("Raju")), (103, String::from("Bhavana"))];
    let get_hashmap = get_hashmap_from_vector_tuple(&vector_tuple);
    println!("\nHashMap: {:?}\nfrom\nvector tuple: {:?}", get_hashmap, &vector_tuple);
    
}

fn  get_hashmap_from_vector_tuple(vector: &Vec<(i32, String)>) -> HashMap<i32, String>{
    let mut map = HashMap::new();
    for (key, value) in vector{
        map.insert(*key, value.to_string());
    }
    return map;
}