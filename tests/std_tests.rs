use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::{
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use streamr::spinlock::Spinlock; // Replace `your_crate` with the actual name of your crate

#[test]
fn test_lock_contention() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    const NUM_THREADS: usize = 4;
    const INCREMENTS_PER_THREAD: usize = 1000;

    let lock = Arc::new(Spinlock::new());
    let lock_clone = Arc::clone(&lock);

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let lock_ref = Arc::clone(&lock_clone);
            thread::spawn(move || {
                for _ in 0..INCREMENTS_PER_THREAD {
                    lock_ref.lock();
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                    lock_ref.unlock();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        COUNTER.load(Ordering::Relaxed),
        NUM_THREADS * INCREMENTS_PER_THREAD
    );
}

#[test]
fn test_lock_unlock_concurrent() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    const NUM_THREADS: usize = 4;

    let lock = Arc::new(Spinlock::new());

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let lock_ref = Arc::clone(&lock);
            thread::spawn(move || {
                lock_ref.lock();
                let current_value = COUNTER.load(Ordering::Relaxed);
                COUNTER.store(current_value + 1, Ordering::Relaxed);
                lock_ref.unlock();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(COUNTER.load(Ordering::Relaxed), NUM_THREADS);
}

#[test]
fn test_simulated_deadlock() {
    let mut child = Command::new("cargo")
        .args(&["run", "--bin", "deadlock_test"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute process");

    // Wait for a reasonable amount of time for the test to run and potentially deadlock
    thread::sleep(Duration::from_secs(2));

    // Check if the child process has finished
    match child.try_wait() {
        Ok(Some(status)) => {
            // Process has finished
            if status.success() {
                panic!("No deadlock detected");
            } else {
                // Deadlock detected
            }
        }
        Ok(None) => {
            // Process is still running, indicating a deadlock
            child.kill().expect("Failed to kill child process");
            child.wait().expect("Failed to wait on child process");
        }
        Err(e) => {
            panic!("Error attempting to wait on child process: {:?}", e);
        }
    }
}
