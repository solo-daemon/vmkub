use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::net::TcpStream;
use ssh2::Session;
use dialoguer::Input;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        println!("Available commands:");
        println!("  connect <path>");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "connect" => connect_client(&args[2..]),
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: {} <command> [args]", args[0]);
        }
    }
}

fn connect_client(args: &[String]) {
    // Prompt the user for the wallet address
    let wallet_address: u64 = Input::new()
        .with_prompt("Enter your wallet address")
        .interact()
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(1);
        });

    println!("Using wallet address as username: {}", wallet_address);

    if args.len() < 1 {
        println!("Error: No private key path provided.");
        return;
    }

    let private_key_path = &args[0];
    
    // Check if the private key file exists
    if !Path::new(private_key_path).exists() {
        println!("Error: Private key file '{}' does not exist.", private_key_path);
        return;
    }

    // Read the private key file
    let mut private_key = String::new();
    if let Err(_) = File::open(private_key_path).and_then(|mut file| file.read_to_string(&mut private_key)) {
        println!("Error: Unable to read private key file '{}'.", private_key_path);
        return;
    }

    let stream = TcpStream::connect(format!("{}:22", "192.168.64.5"));
    match stream {
        Ok(stream) => {
            println!("Connected to the server!");
            let session = Session::new();
            match session {
                Ok(mut session) => {
                    session.set_tcp_stream(stream);
                    session.handshake().unwrap();
                    let auth = session.userauth_password("steve", "password");
                    match auth {
                        Ok(_) => {
                            println!("Authenticated!");
                            let channel = session.channel_session();
                            match channel {
                                Ok(mut channel) => {
                                    channel.exec("whoami").unwrap();
                                    let mut s = String::new();
                                    channel.read_to_string(&mut s).unwrap();
                                    println!("{}", s);
                                    channel.wait_close().unwrap();
                                    let exit_status = channel.exit_status().unwrap();
                                    if exit_status != 0 {
                                        eprint!("Exited with status {}", exit_status);
                                    }
                                }
                                Err(e) => {
                                    eprint!("Failed to create channel: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprint!("Failed to authenticate: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprint!("Failed to create session: {}", e);
                }
            }
        }
        Err(e) => {
            eprint!("Failed to connect: {}", e);
        }
    }
}


