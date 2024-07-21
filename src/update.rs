use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::color::{self, Color};

static VERSION: &str = include_str!("version");
static UPDATE_VERSION_URL: &str =
    "https://raw.githubusercontent.com/drylian/Accelerator/main/src/version";
static DOWNLOAD_URL: &str =
    "https://github.com/drylian/Accelerator/raw/main/src/accelerator-update";

async fn fetch_remote_version(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch the remote version: {}", response.status()).into());
    }

    let version = response.text().await?;
    Ok(version.trim().to_string())
}

async fn download_file(url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch the URL: {}", response.status()).into());
    }

    let mut file = File::create(file_name).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    println!("File downloaded successfully to {}", file_name);
    Ok(())
}

pub async fn update() -> Result<(), Box<dyn Error>> {
    let remote_version = fetch_remote_version(UPDATE_VERSION_URL).await?;
    let local_version = VERSION.trim();

    if remote_version != local_version {
        println!("A new {} is available. Local version: {}, Remote version: {}",color::color(Color::Green, "version"), color::color(Color::Yellow, local_version), color::color(Color::Blue, &remote_version));
        println!("{}",color::color(Color::Yellow, "Downloading new version"));

        download_file(DOWNLOAD_URL, "accelerator-update").await?;
        println!("{}",color::color(Color::Green, "Downloaded"));
    } else {
        println!("The Accelerator is {}. Version: {}",color::color(Color::Green, "up-to-date"), color::color(Color::Green, local_version));
    }

    Ok(())
}
