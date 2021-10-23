use std::fs::File;
use std::io;
use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

pub fn ask_george(file: File) -> Result<()> {
    let client = Client::new();
    let mut response = client
        .post("https://student.cs.uwaterloo.ca/~se212/george/ask-george/cgi-bin/george.cgi/check")
        .body(file)
        .header(CONTENT_TYPE, "text/plain")
        // George can take a long time to respond, so increase the timeout to 30 minutes.
        .timeout(Duration::from_secs(60 * 30))
        .send()
        .context("failed to ask george")?;
    io::copy(&mut response, &mut io::stdout()).context("failed to read response")?;
    Ok(())
}
