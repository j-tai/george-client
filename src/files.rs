use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, Read};

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Directory {
    name: String,
    files: Vec<File>,
}

#[derive(Deserialize)]
struct File {
    name: String,
    path: String,
}

fn get_files() -> Result<Vec<Directory>> {
    let client = Client::new();
    let response = client
        .get("https://student.cs.uwaterloo.ca/~se212/files.json")
        .send()
        .context("failed to get file list")?;
    let files = serde_json::from_reader(response).context("failed to parse json in file list")?;
    Ok(files)
}

fn get_file(f: &File) -> Result<impl Read> {
    let client = Client::new();
    let response = client
        .get(format!("https://student.cs.uwaterloo.ca/~se212{}", f.path))
        .send()
        .with_context(|| format!("failed to download file {:?}", f.path))?;
    Ok(response)
}

pub fn download_files() -> Result<()> {
    println!("Getting file list...");
    let files = get_files()?;
    for directory in files {
        fs::create_dir_all(&directory.name)
            .with_context(|| format!("failed to create dir {:?}", directory.name))?;
        for file in directory.files {
            let out_path = format!("{}/{}", directory.name, file.name);
            let out_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&out_path);
            match out_file {
                Ok(mut out) => {
                    println!("Saving {}...", out_path);
                    io::copy(&mut get_file(&file)?, &mut out).context("failed to write file")?;
                }
                Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
                    println!("Already exists: {}", out_path);
                }
                Err(e) => {
                    eprintln!("Error: Failed to create {:?}: {}", out_path, e);
                }
            }
        }
    }
    Ok(())
}
