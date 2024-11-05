// Threads: Running code concurrently with threads
// Thread is a lightweight version of a process, and in Rust, threads are used to run multiple pieces of code at the same time.
// 1. main thread
// 2. child thread

// Thread methods: spawn, sleep, join, yield_now, current, park, unpark, thread_local, Builder, and more
// To create a thread, we call the thread::spawn function and pass a closure containing the code we want to run in the new thread.
// The thread::spawn function returns a JoinHandle, which is a smart pointer that allows us to wait for the child thread to finish.
// We call the join method on the handle to block the main thread until the child thread has finished its execution.

pub fn thread_demo() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(std::time::Duration::from_millis(1));
    //     }
    // });

    // for i in 1..10 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(std::time::Duration::from_millis(1));
    // }
    // handle.join().unwrap();

    // Thread with closure (move)
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || { // take ownership of v
    //     println!("Here's a vector: {v:?}");
    // });
    // drop(v); // If we added move to the closure, we would move v into the closure’s environment, and we could no longer call drop on it in the main thread.
    // handle.join().unwrap();

    // Using the move keyword with a closure is a way to force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    // This is especially useful when we want to move a value into a closure but the closure is being executed on a different thread.
    // The move keyword is also useful when we want to create a new thread in which a closure uses values from its environment, but we want to force the closure to take ownership of those values.
}
