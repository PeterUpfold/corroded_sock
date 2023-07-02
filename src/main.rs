use std::env;
use std::net::{TcpListener};
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
            eprintln!("{}: Unable to parse the port number. {e:?}", invocation_path.to_string_lossy());
        return ExitCode::from(2);
        }
    }

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind to port");

    println!("Listening on port {}", port);

    for _stream in listener.incoming() {
        match Notification::new()
        .summary(format!("New connection on port {}", port).as_str())
        .body(format!("A new connection was made to port {}", port).as_str())
        .show() {
            Ok(_v) => println!("Dispatched notification"),
            Err(e) => eprintln!("Unable to dispatch notification: {e:?}"),
        }
    }
    
    return ExitCode::from(0);
}
