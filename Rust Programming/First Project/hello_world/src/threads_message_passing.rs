// Message Passing to Transfer data between threads
// To accomplish message-sending concurrency, Rust’s standard library provides an implementation of channels. A channel is a general programming concept by which data is sent from one thread to another.
// The Rust standard library provides an implementation of channels for message passing concurrency. A channel is a way to send a value from one thread to another. One part of the channel, the sender, sends a value, and the other part, the receiver, receives the value.
// A Channel has two halves: the transmitter and the receiver. 
// The transmitter half is used to send a message, and the receiver half is used to receive a message.

use std::{sync::mpsc, thread};

pub fn msg_passing_demo(){
    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        println!("Sending From TX1: {} ........", val);
        tx1.send(val).unwrap();
        // thread::sleep(std::time::Duration::from_secs(1));
    });

    thread::spawn(move || {
        let val = String::from("hello");
        println!("Sending From TX: {} ........", val);
        tx.send(val).unwrap();
        // thread::sleep(std::time::Duration::from_secs(1));
    });
    
    
    // let recieved = rx.recv().unwrap();
    // println!("Recieved: {}", recieved);

    for recieved in rx {
        println!("Recieved: {}", recieved);
    }


}