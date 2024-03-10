use clap::{Parser, Subcommand};
use console::{style, Emoji};
use std::io;
use std::{error::Error, path::PathBuf, thread};

use vm_launcher::{provider::add_provider_to_network, vm::Specifications};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "ISO")]
    iso: PathBuf,
}

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

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static WORLD: Emoji<'_, '_> = Emoji("🌐", "");
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Welcome to Our Network! {}", WORLD);
    println!(
        "{} We are thrilled to have you as a provider in our network.",
        SPARKLE
    );
    println!("Please provide the following details");
    println!("Please enter the amount of RAM (in GB):");
    let ram: u32 = get_integer_input();
    println!("Please enter the number of CPU cores:");
    let cpu_cores: u32 = get_integer_input();
    println!("Please enter the amount of storage (in GB):");
    let storage: u32 = get_integer_input();
    println!("Please enter your wallet address:");
    let mut wall_addr = String::new();

    io::stdin()
        .read_line(&mut wall_addr)
        .expect("Failed to read line");
    let handle = thread::spawn(move || {
        println!("Starting vm...");
        let specifications =
            Specifications::get_specifications(cpu_cores.clone(), ram.clone(), storage.clone());
        let _ = specifications.run_vm(cli.iso);
    });
    let handle2 = thread::spawn(move || {
        println!("Adding you to the network");
        add_provider_to_network(storage, ram, cpu_cores, wall_addr);
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    Ok(())
}
