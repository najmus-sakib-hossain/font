//! Search fonts example
//!
//! This example demonstrates how to search for fonts using dx-font.
//!
//! Run with: cargo run --example search_fonts

use anyhow::Result;
use dx_font::search::FontSearch;
use dx_font::models::FontCategory;

#[tokio::main]
async fn main() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                  dx-font Search Example");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Initialize the font search engine
    let search = FontSearch::new()?;
    
    // Example 1: Basic search
    println!("📍 Example 1: Searching for 'roboto'...\n");
    let results = search.search("roboto").await?;
    
    println!("Found {} fonts matching 'roboto'\n", results.total);
    println!("JSON Response:");
    println!("─────────────────────────────────────────────────────────────────");
    
    // Print first 5 results as JSON
    let sample: Vec<_> = results.fonts.iter().take(5).collect();
    println!("{}", serde_json::to_string_pretty(&sample)?);
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // Example 2: Search for a specific font
    println!("📍 Example 2: Searching for 'inter'...\n");
    let results = search.search("inter").await?;
    
    println!("Found {} fonts matching 'inter'\n", results.total);
    for font in results.fonts.iter().take(10) {
        println!(
            "  • {} ({}) - {} variants",
            font.name,
            font.provider.name(),
            font.variant_count
        );
    }
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // Example 3: Search for monospace fonts
    println!("📍 Example 3: Searching for 'mono'...\n");
    let results = search.search("mono").await?;
    
    println!("Found {} fonts matching 'mono'\n", results.total);
    for font in results.fonts.iter().take(10) {
        let category = font.category
            .as_ref()
            .map(|c| format!("{:?}", c))
            .unwrap_or_else(|| "Unknown".to_string());
        
        println!(
            "  • {} ({}) - Category: {}",
            font.name,
            font.provider.name(),
            category
        );
    }
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // Example 4: Get font statistics
    println!("📍 Example 4: Font Statistics\n");
    let stats = search.get_stats().await?;
    
    println!("Statistics JSON:");
    println!("{}", serde_json::to_string_pretty(&stats)?);
    
    println!("\n─────────────────────────────────────────────────────────────────\n");
    
    // Example 5: Health check
    println!("📍 Example 5: Provider Health Check\n");
    let health = search.health_check().await;
    
    for (provider, is_healthy) in health {
        let status = if is_healthy { "✅" } else { "❌" };
        println!("  {} {}", status, provider);
    }
    
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    Search Example Complete");
    println!("═══════════════════════════════════════════════════════════════");
    
    Ok(())
}
