//! Download fonts example
//!
//! This example demonstrates how to download fonts using dx-font.
//!
//! Run with: cargo run --example download_fonts

use anyhow::Result;
use dx_font::download::FontDownloader;
use dx_font::search::FontSearch;
use dx_font::models::{FontProvider, DownloadOptions};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                 dx-font Download Example");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Create output directory
    let output_dir = PathBuf::from("./playground/downloaded_fonts");
    std::fs::create_dir_all(&output_dir)?;
    
    // Initialize the downloader
    let downloader = FontDownloader::new()?;
    
    // Example 1: Download a Google Font using Google Webfonts Helper
    println!("📍 Example 1: Downloading 'Roboto' from Google Fonts...\n");
    
    match downloader.download_google_font(
        "roboto",
        &output_dir,
        &["ttf", "woff2"],
        &["latin"],
    ).await {
        Ok(path) => {
            println!("✅ Downloaded to: {}\n", path.display());
        }
        Err(e) => {
            println!("⚠️  Download failed (this might be expected in some environments): {}\n", e);
        }
    }
    
    // Example 2: Download a font from Fontsource CDN
    println!("📍 Example 2: Downloading 'Inter' from Fontsource CDN...\n");
    
    match downloader.download_fontsource_font(
        "inter",
        &output_dir,
        400,
        "normal",
    ).await {
        Ok(path) => {
            println!("✅ Downloaded to: {}\n", path.display());
        }
        Err(e) => {
            println!("⚠️  Download failed: {}\n", e);
        }
    }
    
    // Example 3: Download multiple weights
    println!("📍 Example 3: Downloading multiple weights of 'Open Sans'...\n");
    
    let weights = [300, 400, 600, 700];
    for weight in weights {
        match downloader.download_fontsource_font(
            "open-sans",
            &output_dir,
            weight,
            "normal",
        ).await {
            Ok(path) => {
                println!("  ✅ Weight {}: {}", weight, path.display());
            }
            Err(e) => {
                println!("  ⚠️  Weight {}: Failed - {}", weight, e);
            }
        }
    }
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // Example 4: Search then download
    println!("📍 Example 4: Search for 'fira' then download...\n");
    
    let search = FontSearch::new()?;
    let results = search.search("fira").await?;
    
    println!("Found {} fonts matching 'fira':", results.total);
    for font in results.fonts.iter().take(5) {
        println!("  • {} ({})", font.name, font.provider.name());
    }
    
    // Download the first result if available
    if let Some(font) = results.fonts.first() {
        println!("\nAttempting to download '{}'...", font.name);
        
        // Convert font id to download-friendly format
        let font_id = font.id.to_lowercase().replace(' ', "-");
        
        match downloader.download_google_font(
            &font_id,
            &output_dir,
            &["ttf"],
            &["latin"],
        ).await {
            Ok(path) => {
                println!("✅ Downloaded to: {}", path.display());
            }
            Err(e) => {
                println!("⚠️  Download failed: {}", e);
            }
        }
    }
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // List downloaded files
    println!("📁 Downloaded fonts in {}:", output_dir.display());
    
    if let Ok(entries) = std::fs::read_dir(&output_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                println!("  • {} ({} bytes)", name.to_string_lossy(), size);
            }
        }
    }
    
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                   Download Example Complete");
    println!("═══════════════════════════════════════════════════════════════");
    
    Ok(())
}
