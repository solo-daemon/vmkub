mod connect;

use connect::connect_client;

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



