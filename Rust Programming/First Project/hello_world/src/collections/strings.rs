// String: A collection of characters. It is a sequence of Unicode scalar values encoded as a stream of UTF-8 bytes. It is a growable, heap-allocated data structure. It is a wrapper around a Vec<u8> (a vector of bytes). It is a UTF-8 encoded string.

pub fn string_demo(){
    let name = String::from("Hello Rusteceans!");
    println!("{}", name);

    let another_name = &name;
    println!("{}", another_name);

    let str = "Hello World!";
    println!("{}", str);

    let word = String::from("Honnur Rusteceans!");
    let ans = find_first_word(&word);
    println!("First word from the string \"{}\", is: {}", word, ans);
}

fn find_first_word(word: &String) -> &str{
    let mut index = 0;
    for (_, i) in word.chars().enumerate(){
        if i == ' '{
            break;
        }
        index = index + 1;
    }

    return &word[0..index]; // slice of the string
}
