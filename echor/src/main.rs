use std::{env, thread};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;

type MyError = Result<(), Box<dyn Error>>;

fn main() -> MyError {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;

    Ok(())
}

fn echo_server(address: &str) -> MyError {
    let listener = TcpListener::bind(address)?;
    loop {
        let (mut stream, pier) = listener.accept()?;
        println!("Accept from {}.", pier);
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    return;
                }
                print!("{}", std::str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
