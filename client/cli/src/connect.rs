use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::net::TcpStream;
use ssh2::Session;
use dialoguer::Input;
type _EthereumAddress = [u8; 20];

pub fn connect_client(args: &[String]) {
    // Prompt the user for the wallet address
    let wallet_address: String = Input::new()
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
    // let mut private_key = String::new();
    // if let Err(_) = File::open(private_key_path).and_then(|mut file| file.read_to_string(&mut private_key)) {
    //     println!("Error: Unable to read private key file '{}'.", private_key_path);
    //     return;
    // }

    let stream: Result<TcpStream, std::io::Error> = TcpStream::connect(format!("{}:22", "192.168.121.231"));
    match stream {
        Ok(stream) => {
            println!("Connected to the server!");
            let session: Result<Session, ssh2::Error> = Session::new();
            match session {
                Ok(mut session) => {
                    session.set_tcp_stream(stream);
                    session.handshake().unwrap();
                    let auth = session.userauth_pubkey_file("apps",None,Path::new(private_key_path),None);
                    match auth {
                        Ok(_) => {
                            println!("Authenticated!");
                            let channel: Result<ssh2::Channel, ssh2::Error> = session.channel_session();
                            match channel {
                                Ok(mut channel) => {

                                    channel.exec("sh").unwrap();
    
                                    // Read input from the user and send it to the remote server
                                    let stdin = std::io::stdin();
                                    let mut input = String::new();
                                    let hlp = "\n \n $ ";
                                    loop {
                                        println!("{}",hlp);
                                        input.clear();
                                        stdin.read_line(&mut input).unwrap();
                                        channel.write_all(input.as_bytes()).unwrap();
                                        
                                        // Read output from the remote server and display it to the user
                                        let mut buffer = [0; 4096];
                                        match channel.read(&mut buffer) {
                                            Ok(n) => {
                                                if n == 0 {
                                                    break;
                                                }
                                                let output = std::str::from_utf8(&buffer[..n]).unwrap();
                                                print!("{}", output);
                                            }
                                            Err(_) => break,
                                        }
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

// fn send_command(channel: &mut Channel, command: &str) -> std::io::Result<()> {
//     channel.write_all(command.as_bytes())?;
//     channel.write_all(b"\n")?;
//     Ok(())
// }

// // Function to read output from the remote shell
// fn read_output(channel: &mut Channel) -> std::io::Result<String> {
//     let mut output = String::new();
//     channel.read_to_string(&mut output)?;
//     Ok(output)
// }

