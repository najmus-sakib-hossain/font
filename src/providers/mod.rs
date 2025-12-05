//! Font provider implementations
//!
//! This module contains implementations for various font providers/sources.

pub mod google_fonts;
pub mod bunny_fonts;
pub mod fontsource;
pub mod fontshare;
pub mod font_library;
pub mod github_fonts;
pub mod dafont;
pub mod fontspace;
pub mod fonts1001;
pub mod fontsquirrel;

use async_trait::async_trait;
use crate::models::{FontFamily, Font, SearchQuery, SearchResults};
use anyhow::Result;
use futures::future::join_all;
use tokio::time::Instant;

/// Trait that all font providers must implement
#[async_trait]
pub trait FontProviderTrait: Send + Sync {
    /// Get the name of this provider
    fn name(&self) -> &str;
    
    /// Get the base URL for this provider
    fn base_url(&self) -> &str;
    
    /// Search for fonts matching the query
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Font>>;
    
    /// Get all available fonts from this provider
    async fn list_all(&self) -> Result<Vec<Font>>;
    
    /// Get detailed information about a specific font family
    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily>;
    
    /// Get download URL for a font
    async fn get_download_url(&self, font_id: &str) -> Result<String>;
    
    /// Check if the provider is available/responding
    async fn health_check(&self) -> Result<bool>;
}

/// Create an HTTP client with default settings
pub fn create_http_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent(format!("dx-font/{}", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    Ok(client)
}

/// Registry of all available font providers
pub struct ProviderRegistry {
    providers: Vec<Box<dyn FontProviderTrait>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    
    pub fn with_defaults() -> Result<Self> {
        let client = create_http_client()?;
        let mut registry = Self::new();
        
        // Add all providers for maximum font coverage (50k+ fonts)
        registry.register(Box::new(google_fonts::GoogleFontsProvider::new(client.clone())));
        registry.register(Box::new(bunny_fonts::BunnyFontsProvider::new(client.clone())));
        registry.register(Box::new(fontsource::FontsourceProvider::new(client.clone())));
        registry.register(Box::new(fontshare::FontshareProvider::new(client.clone())));
        registry.register(Box::new(font_library::FontLibraryProvider::new(client.clone())));
        registry.register(Box::new(github_fonts::GitHubFontsProvider::new(client.clone())));
        registry.register(Box::new(dafont::DafontProvider::new(client.clone())));
        registry.register(Box::new(fontspace::FontSpaceProvider::new(client.clone())));
        registry.register(Box::new(fonts1001::Fonts1001Provider::new(client.clone())));
        registry.register(Box::new(fontsquirrel::FontSquirrelProvider::new(client.clone())));
        
        Ok(registry)
    }
    
    pub fn register(&mut self, provider: Box<dyn FontProviderTrait>) {
        self.providers.push(provider);
    }
    
    pub fn providers(&self) -> &[Box<dyn FontProviderTrait>] {
        &self.providers
    }
    
    pub async fn search_all(&self, query: &SearchQuery) -> Result<SearchResults> {
        let mut all_fonts = Vec::new();
        let mut providers_searched = Vec::new();
        
        for provider in &self.providers {
            providers_searched.push(provider.name().to_string());
            match provider.search(query).await {
                Ok(fonts) => all_fonts.extend(fonts),
                Err(e) => {
                    tracing::warn!("Error searching {}: {}", provider.name(), e);
                }
            }
        }
        
        // Apply limit if specified
        let total = all_fonts.len();
        if let Some(limit) = query.limit {
            all_fonts.truncate(limit);
        }
        
        Ok(SearchResults {
            fonts: all_fonts,
            total,
            query: query.query.clone(),
            providers_searched,
        })
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
