use std::env;
use std::fs::File;
use std::process::exit;

use anyhow::{Context, Result};

mod ask;

fn try_main(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        eprintln!("Usage: george-client <filename>");
        exit(2);
    }
    let file = File::open(&args[0]).with_context(|| format!("failed to open {:?}", args[0]))?;
    ask::ask_george(file)?;
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    match try_main(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
