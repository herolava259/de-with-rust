use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;



fn demo_read_write_exclusive_lock()
{
    let mut handles = vec![];

    let data = Arc::new(RwLock::new(vec![1,2,3]));

    // demo read

    for i in 0..5
    {
        let lock_clone = Arc::clone(&data);
        let handle = thread::spawn(move ||{
            let dat = lock_clone.read().unwrap();

            for u in dat.iter()
            {
                println!("Reader {} read value {}, now holding lock...", i, *u);
            }

            thread::sleep(Duration::from_secs(1));

            println!("Reader {}: Dropping lock. ", i);
        });

        handles.push(handle);
    }


    let handle = thread::spawn(move || {
        println!("Writer is wating for acquire critical data");

        let mut writable_data = data.write().unwrap();

        println!("The writer acquired to the data");

        println!("Writer proceeds...");

        for u in (*writable_data).iter_mut()
        {
            *u += 1;
        }
    });

    for handle in handles {
        match handle.join() {
            Ok(_) => println!("Modification a variable successfully"),
            Err(_) => println!("Thread panicked"),
        }
    }

}
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
