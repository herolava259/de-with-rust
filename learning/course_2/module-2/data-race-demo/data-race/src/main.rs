use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut handles = vec![];

    for i in 0..2 {
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut dat = data.lock().unwrap();
            dat[i] += 1
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => println!("Modification a variable successfully"),
            Err(_) => println!("Thread panicked"),
        }
    }
}
