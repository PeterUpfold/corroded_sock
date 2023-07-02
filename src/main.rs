use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::ExitCode;
use notify_rust::Notification;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let invocation_path = std::env::current_exe().expect("Unable to get current program name");

    if args.len() != 2 {
        eprintln!("Usage: {} <port>", invocation_path.to_string_lossy());
        return ExitCode::from(1);
    }

    let port: u16;
    
    match args[1].parse::<u16>() {
        Ok(v) => port = v,
        Err(e) => {
            eprintln!("{}: Unable to parse the port number.", invocation_path.to_string_lossy());
        return ExitCode::from(2);
        }
    }

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind to port");

    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        Notification::new()
        .summary("New connection on port XX")
        .body("A new connection was made to port XX")
        .show();
    }
    
    return ExitCode::from(0);
}
