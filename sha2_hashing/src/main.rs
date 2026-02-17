use lib::*;
use sha2::{Sha256, Sha512};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough args");
    }

    if args.len() >= 3 && args[2] == "512" {
        if let Ok(hash_array) = hash_file::<Sha512>(&args[1]) {
            print!("{}", hex_to_string(&hash_array))
        }
    } else {
        if let Ok(hash_array) = hash_file::<Sha256>(&args[1]) {
            print!("{}", hex_to_string(&hash_array));
        }
    }

    println!("  {}", args[1]);
}
