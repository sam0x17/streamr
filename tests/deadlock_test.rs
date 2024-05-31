use std::sync::Arc;
use std::thread;
use streamr::spinlock::Spinlock;

fn main() {
    let lock = Arc::new(Spinlock::new());
    let lock_ref = Arc::clone(&lock);

    let handle = thread::spawn(move || {
        lock_ref.lock();
        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));
        lock_ref.lock(); // This will cause a deadlock
        lock_ref.unlock();
        lock_ref.unlock();
    });

    // Give the thread some time to run
    thread::sleep(std::time::Duration::from_secs(2));

    // Check if the handle is still running (indicating a deadlock)
    if handle.is_finished() {
        println!("Test passed, no deadlock detected");
    } else {
        eprintln!("Test failed, potential deadlock detected");
        std::process::exit(1);
    }
}
