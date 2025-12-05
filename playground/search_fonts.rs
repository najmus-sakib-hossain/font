//! Search fonts example
//!
//! This example demonstrates how to search for fonts using dx-font.
//!
//! Run with: cargo run --example search_fonts

use anyhow::Result;
use dx_font::search::FontSearch;

#[tokio::main]
async fn main() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("                       dx-font Search Example");
    println!("              Access 50,000+ Commercial-Free Fonts!");
    println!("═══════════════════════════════════════════════════════════════════════\n");
    
    // Initialize the font search engine
    let search = FontSearch::new()?;
    
    // Example 1: Get font statistics first
    println!("📍 Example 1: Font Statistics\n");
    let stats = search.get_stats().await?;
    
    println!("╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                     dx-font LIBRARY STATISTICS                        ║");
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    println!("║  Total Indexed Fonts:  {:>6}                                        ║", stats.total_fonts);
    println!("║  Active Providers:     {:>6}                                        ║", stats.providers_count);
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    println!("║  Serif:                {:>6}                                        ║", stats.serif_count);
    println!("║  Sans-Serif:           {:>6}                                        ║", stats.sans_serif_count);
    println!("║  Display:              {:>6}                                        ║", stats.display_count);
    println!("║  Handwriting:          {:>6}                                        ║", stats.handwriting_count);
    println!("║  Monospace:            {:>6}                                        ║", stats.monospace_count);
    println!("║  Uncategorized:        {:>6}                                        ║", stats.uncategorized_count);
    println!("╚═══════════════════════════════════════════════════════════════════════╝");
    
    println!("\nProviders: {}\n", stats.providers.join(", "));
    
    println!("─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 2: Basic search
    println!("📍 Example 2: Searching for 'roboto'...\n");
    let results = search.search("roboto").await?;
    
    println!("Found {} fonts matching 'roboto'\n", results.total);
    println!("JSON Response (first 3):");
    println!("─────────────────────────────────────────────────────────────────────────");
    
    let sample: Vec<_> = results.fonts.iter().take(3).collect();
    println!("{}", serde_json::to_string_pretty(&sample)?);
    
    println!("\n─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 3: Search for monospace/coding fonts
    println!("📍 Example 3: Searching for coding fonts ('mono', 'code')...\n");
    let results = search.search("mono").await?;
    
    println!("Found {} fonts matching 'mono'\n", results.total);
    println!("Top 15 Monospace Fonts:");
    for (i, font) in results.fonts.iter().take(15).enumerate() {
        let category = font.category
            .as_ref()
            .map(|c| format!("{:?}", c))
            .unwrap_or_else(|| "Unknown".to_string());
        
        println!(
            "  {:2}. {:30} ({}) - {}",
            i + 1,
            font.name,
            font.provider.name(),
            category
        );
    }
    
    println!("\n─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 4: Search for display fonts
    println!("📍 Example 4: Searching for display fonts...\n");
    let results = search.search("display").await?;
    
    println!("Found {} fonts matching 'display'\n", results.total);
    for (i, font) in results.fonts.iter().take(10).enumerate() {
        println!(
            "  {:2}. {:30} ({})",
            i + 1,
            font.name,
            font.provider.name()
        );
    }
    
    println!("\n─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 5: Search for handwriting fonts
    println!("📍 Example 5: Searching for handwriting fonts...\n");
    let results = search.search("script").await?;
    
    println!("Found {} fonts matching 'script'\n", results.total);
    for (i, font) in results.fonts.iter().take(10).enumerate() {
        println!(
            "  {:2}. {:30} ({})",
            i + 1,
            font.name,
            font.provider.name()
        );
    }
    
    println!("\n─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 6: Search for international fonts
    println!("📍 Example 6: Searching for international fonts (CJK, Arabic, etc.)...\n");
    
    for query in ["noto", "arabic", "chinese", "japanese", "korean", "hebrew"] {
        let results = search.search(query).await?;
        println!("  '{}': {} fonts found", query, results.total);
    }
    
    println!("\n─────────────────────────────────────────────────────────────────────────\n");
    
    // Example 7: Provider health check
    println!("📍 Example 7: Provider Health Check\n");
    let health = search.health_check().await;
    
    for (provider, is_healthy) in health {
        let status = if is_healthy { "✅ Online" } else { "❌ Offline" };
        println!("  {:20} {}", provider, status);
    }
    
    println!("\n═══════════════════════════════════════════════════════════════════════");
    println!("                      Search Example Complete!");
    println!("═══════════════════════════════════════════════════════════════════════");
    
    Ok(())
}
