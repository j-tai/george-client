use std::env;
use std::fs::File;
use std::process::exit;

use anyhow::{Context, Result};

mod ask;
mod files;

fn try_main(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        eprintln!(
            "Usage:
    george-client <filename>  - get feedback for a file
    george-client --download  - download all files"
        );
        exit(2);
    }
    if args[0] == "--download" {
        files::download_files()?;
    } else {
        let file = File::open(&args[0]).with_context(|| format!("failed to open {:?}", args[0]))?;
        ask::ask_george(file)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    match try_main(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {:?}", e);
            exit(1);
        }
    }
}
