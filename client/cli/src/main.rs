use console::Emoji;
use kademlia_dht::attributes::Query;
use std::io;
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static WORLD: Emoji<'_, '_> = Emoji("🌐", "");
pub mod lib;

fn get_integer_input() -> u32 {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(value) => return value,
                Err(_) => println!("Please enter a valid integer."),
            },
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    println!("Welcome to Our Network! {}", WORLD);
    println!(
        "{} Thank you for considering our network for your requirements.",
        SPARKLE
    );
    println!("We're eager to cater to your needs and provide top-notch services.");

    println!("To begin, please provide the following details:");

    println!("Please specify the amount of RAM you need (in GB):");

    let ram: u32 = get_integer_input();
    println!("Please indicate the desired number of CPU cores:");

    let cpu_cores: u32 = get_integer_input();
    println!("Please enter the required storage capacity (in GB):");

    let storage: u32 = get_integer_input();

    println!("Requesting network changes to accommodate your specifications...");
    let interface = lib::add_client_to_network(storage, ram, cpu_cores);
    let req = Query {
        storage: interface.node.info.storage,
        ram: interface.node.info.ram,
        cpu_cores: interface.node.info.cpu_cores,
        arch_images: interface.node.info.arch_images,
    };
    let best_fit = interface.get_best_fit(req);
    dbg!(best_fit);
    println!(
        "You're now equipped to access and benefit from our network resources. Welcome aboard!"
    );
}
