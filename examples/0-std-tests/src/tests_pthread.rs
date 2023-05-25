use anyhow::{anyhow, Result, Context};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock, mpsc, Condvar},
    thread,
};
use super::assert;

pub fn test_thread_mutex() -> Result<()> {
    let map = Arc::new(Mutex::new(HashMap::<u32, u64>::new()));
    {
        let map = map.lock().expect("Dirty mutex");
        assert(map.len() == 0, "map is not empty")?
    }

    let threads = (0..10)
        .map(|_| {
            let map = Arc::clone(&map);
            thread::Builder::new()
                .stack_size(128 * 1024)
                .spawn(move || {
                    // Mutex contention
                    for i in 0..100 {
                        let v = rand::random::<u64>() % 1000;
                        let mut map = map.lock().expect("Dirty mutex");
                        let old = map.get(&i).cloned().unwrap_or(0);
                        map.insert(i, v + old);
                    }
                })
                .map_err(|_| anyhow!("Unable to spawn thread"))
        })
        .collect::<Result<Vec<_>>>()?;

    for t in threads {
        t.join().map_err(|_| anyhow!("Unable to join thread"))?;
    }

    let map = map.lock().expect("Dirty mutex!");

    assert(map.len() == 100, "map should have exactly 100 keys")?;

    Ok(())
}

pub fn test_drop_mutex_from_another_thread() -> Result<()> {
    let mutex = Mutex::new(0u64);
    {
        let mut v = mutex.lock().expect("Dirty mutex");
        *v = 10;
    }

    thread::Builder::new()
        .stack_size(128 * 1024)
        .spawn(move || -> Result<()> {
            {
                let mut v = mutex.lock().expect("Dirty mutex");
                *v += 1;
                assert(*v == 11, "Should be 10")?;
            }
            drop(mutex);
            Ok(())
        })
        .map_err(|_| anyhow!("Unable to spawn thread"))?
        .join()
        .map_err(|_| anyhow!("Unable to join thread"))??;

    Ok(())
}

pub fn test_condvar() -> Result<()> {
    const NUM_THREADS: u32 = 10;
    let pair = Arc::new((Mutex::new(0), Condvar::new()));

    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let pair2 = pair.clone();
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().expect("dirty mutex");
            *started += 1;
            cvar.notify_one();
        });
        handles.push(handle);
    }

    {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().expect("dirty mutex");
        while *started < NUM_THREADS {
            started = cvar.wait(started).expect("dirty condvar");
        }
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow!("unable to join thread"))?;
    }

    let (lock, _) = &*pair;
    let final_val = lock.lock().unwrap();
    assert(*final_val == NUM_THREADS, "final value muse be equal to 10")?;

    Ok(())
}

pub fn test_semaphore() -> Result<()> {
    const NUM_THREADS: usize = 10;
    const NUM_RESOURCES: usize = 5;

    // Create a channel with a bounded capacity
    let (tx, rx) = mpsc::sync_channel(NUM_RESOURCES);
    let tx = Arc::new(Mutex::new(tx));

    let handles = (0..NUM_THREADS)
        .map(|_| {
            let tx = Arc::clone(&tx);
            thread::spawn(move || {
                let tx = tx.lock().expect("Dirty mutex");
                tx.send(()).unwrap();
            })
        })
        .collect::<Vec<_>>();
    for _ in 0..NUM_THREADS {
        rx.recv().context("recv error")?;
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("join error"))?;
    }

    assert(rx.try_recv().is_err(), "recv is not error")?;

    Ok(())
}

pub fn test_rwlock() -> Result<()> {
    let data = Arc::new(RwLock::new(0));
    {
        let data = data.read().expect("Dirty mutex");
        assert(*data == 0, "map is not empty")?;
    }
    let threads = (0..10)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.write().expect("Dirty RwLock");
                *data += 1;
            })
        })
        .collect::<Vec<_>>();

    for t in threads {
        t.join().map_err(|_| anyhow!("join error"))?;
    }

    {
        let data = data.read().unwrap();
        assert(*data == 10, "counter is not 10")?;
    }

    Ok(())
}
