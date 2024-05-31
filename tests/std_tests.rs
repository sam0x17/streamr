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
use streamr::channel::Channel;
use streamr::spinlock::Spinlock;

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

#[test]
fn test_send_receive_multiple_elements() {
    let channel = Arc::new(Channel::<u32, 3>::new());

    channel.send(1);
    channel.send(2);
    channel.send(3);

    assert_eq!(channel.receive(), 1);
    assert_eq!(channel.receive(), 2);
    assert_eq!(channel.receive(), 3);
}

#[test]
fn test_send_blocking() {
    let channel = Arc::new(Channel::<u32, 3>::new());

    // Fill the channel
    channel.send(1);
    channel.send(2);
    channel.send(3);

    let channel_clone = Arc::clone(&channel);
    let handle = std::thread::spawn(move || {
        channel_clone.send(4); // This should block until there's space
    });

    std::thread::sleep(std::time::Duration::from_millis(100)); // Allow some time for the thread to block

    // Receive an item to make space
    assert_eq!(channel.receive(), 1);

    // The send should now complete
    handle.join().expect("Thread panicked");

    assert_eq!(channel.receive(), 2);
    assert_eq!(channel.receive(), 3);
    assert_eq!(channel.receive(), 4);
}

#[test]
fn test_receive_blocking() {
    let channel = Arc::new(Channel::<u32, 3>::new());

    let channel_clone = Arc::clone(&channel);
    let handle = std::thread::spawn(move || {
        let result = channel_clone.receive(); // This should block until there's an item
        assert_eq!(result, 1);
    });

    std::thread::sleep(std::time::Duration::from_millis(100)); // Allow some time for the thread to block

    // Send an item to unblock the receive
    channel.send(1);

    // The receive should now complete
    handle.join().expect("Thread panicked");
}

#[test]
fn test_concurrent_send_receive() {
    let channel = Arc::new(Channel::<u32, 3>::new());

    let send_channel = Arc::clone(&channel);
    let send_handle = std::thread::spawn(move || {
        for i in 0..10 {
            send_channel.send(i);
        }
    });

    let receive_channel = Arc::clone(&channel);
    let receive_handle = std::thread::spawn(move || {
        for i in 0..10 {
            let result = receive_channel.receive();
            assert_eq!(result, i);
        }
    });

    send_handle.join().expect("Send thread panicked");
    receive_handle.join().expect("Receive thread panicked");
}

#[test]
fn test_channel_capacity() {
    let channel = Arc::new(Channel::<u32, 3>::new());

    // Fill the channel
    channel.send(1);
    channel.send(2);
    channel.send(3);

    // Ensure the channel is full by checking if another send would block
    let channel_clone = Arc::clone(&channel);
    let send_attempt = std::thread::spawn(move || {
        channel_clone.send(4); // This should block
    });

    std::thread::sleep(std::time::Duration::from_millis(100)); // Allow some time for the thread to block
    assert_eq!(send_attempt.is_finished(), false);

    // Receive one item to make space
    assert_eq!(channel.receive(), 1);

    // The send should now complete
    send_attempt.join().expect("Thread panicked");
}
