use anyhow::{anyhow, Context, Result};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::{
    io::{Read, Write},
    time::Duration,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::assert;

fn handle_client(mut stream: TcpStream, counter: Arc<Mutex<i32>>) {
    let mut buffer = [0; 128];
    while match stream.read(&mut buffer) {
        Ok(size) => match &buffer[..size] {
            b"ADD" => {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                true
            }
            b"END" => {
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
            _ => true,
        },
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn test_tcp() -> Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    let server_handle = {
        let counter = Arc::clone(&counter);
        let tx = tx.clone();

        thread::spawn(move || {
            let listener = TcpListener::bind("localhost:12345").unwrap();
            tx.send(()).unwrap(); // send message indicating server is ready

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let counter = Arc::clone(&counter);
                        thread::spawn(move || {
                            handle_client(stream, counter);
                        });
                    }
                    Err(e) => {
                        println!("Server failed: {}", e);
                    }
                }
            }
        })
    };

    // Wait until server starts
    rx.recv().unwrap();

    {
        let mut stream =
            TcpStream::connect("localhost:12345").context("unable to connect to server")?;
        for _ in 0..4 {
            let _ = stream.write(b"ADD");
            let _ = stream.flush();
            thread::sleep(Duration::from_millis(100));
        }

        let _ = stream.write(b"END");
        let _ = stream.flush();
    }

    server_handle
        .join()
        .map_err(|_| anyhow!("unable to join server thread"))?;

    let counter = counter.lock().unwrap();

    assert(*counter == 4, "counter should be 4")?;

    Ok(())
}
