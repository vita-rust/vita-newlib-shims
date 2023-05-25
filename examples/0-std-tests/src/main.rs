#![feature(fs_try_exists)]
use anyhow::Result;
use anyhow::bail;
use tests_fs::*;
use tests_pthread::*;
use tests_tcp::*;

mod tests_pthread;
mod tests_fs;
mod tests_tcp;

pub fn assert(res: bool, error: &'static str) -> Result<()> {
    if !res {
        bail!(error)
    }

    Ok(())
}

pub fn main() -> Result<()> {
    fs_cleanup();

    fn run() -> Result<()> {
        println!("Running test_fs_read_dir");
        test_fs_read_dir()?;
        println!("Running test_fs_creation");
        test_fs_creation()?;
        println!("Running test_thread_mutex");
        test_thread_mutex()?;
        println!("Running test_drop_mutex_from_another_thread");
        test_drop_mutex_from_another_thread()?;
        println!("Running test_condvar");
        test_condvar()?;
        println!("Running test_semaphore");
        test_semaphore()?;
        println!("Running test_rwlock");
        test_rwlock()?;
        println!("Running test_tcp");
        test_tcp()?;
        Ok(())
    }


    let res = run();
    println!("Tests passed!");

    fs_cleanup();
    res
}
