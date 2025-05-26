#! /usr/bin/env nix-shell
//! ```cargo
//! [dependencies]
//! clap = { version = "4.5.38", features = ["derive"] }
//! ```
/*
#! nix-shell -i rust-script -p cargo rustc rust-script -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/c596c2f0ce95e3f4853656dec7afd741ab32c89c.tar.gz
*/

use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    eprintln!("Waiting for a letter in stdin...");
    let mut letter = String::new();
    io::stdin().read_to_string(&mut letter)?;

    for _ in 0..args.count {
        println!("Hello {}! Here's a letter for you:", args.name);
        println!("```");
        println!("{}", letter.trim());
        println!("```");
    }

    Ok(())
}
