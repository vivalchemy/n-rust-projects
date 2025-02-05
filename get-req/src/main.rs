use anyhow::{Context, Result};
use reqwest;
use std::io::Read;

fn main() -> Result<()> {
    let mut res =
        reqwest::blocking::get("https://httpbin.org/get").context("Failed to get resource")?;

    let mut body = String::new();
    res.read_to_string(&mut body)
        .context("Failed to read response body")?;

    println!("Status: {}", res.status());
    println!("Body:\n{}", body);
    println!("Headers:{:#?}", res.headers());

    Ok(())
}
