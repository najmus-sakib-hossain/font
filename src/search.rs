//! Font search functionality
//!
//! Provides unified search across all font providers.

use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;

use crate::models::{FontFamily, FontProvider, SearchQuery, SearchResults, FontCategory};
use crate::providers::ProviderRegistry;

/// Main font search engine
pub struct FontSearch {
    registry: Arc<ProviderRegistry>,
}

impl FontSearch {
    /// Create a new font search engine with default providers
    pub fn new() -> Result<Self> {
        let registry = ProviderRegistry::with_defaults()?;
        Ok(Self {
            registry: Arc::new(registry),
        })
    }
    
    /// Create with a custom registry
    pub fn with_registry(registry: ProviderRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }
    
    /// Search for fonts matching the query across all providers
    pub async fn search(&self, query: &str) -> Result<SearchResults> {
        let search_query = SearchQuery {
            query: query.to_string(),
            ..Default::default()
        };
        
        self.registry.search_all(&search_query).await
    }
    
    /// Search with advanced options
    pub async fn search_advanced(&self, query: SearchQuery) -> Result<SearchResults> {
        self.registry.search_all(&query).await
    }
    
    /// Search for fonts by category
    pub async fn search_by_category(&self, category: FontCategory) -> Result<SearchResults> {
        let query = SearchQuery {
            query: String::new(),
            category: Some(category),
            ..Default::default()
        };
        
        let mut results = self.registry.search_all(&query).await?;
        
        // Filter by category
        results.fonts = results.fonts
            .into_iter()
            .filter(|f| f.category.as_ref() == Some(&query.category.as_ref().unwrap()))
            .collect();
        results.total = results.fonts.len();
        
        Ok(results)
    }
    
    /// List all available fonts from all providers
    pub async fn list_all(&self) -> Result<SearchResults> {
        let mut all_fonts = Vec::new();
        let mut providers_searched = Vec::new();
        
        for provider in self.registry.providers() {
            providers_searched.push(provider.name().to_string());
            match provider.list_all().await {
                Ok(fonts) => all_fonts.extend(fonts),
                Err(e) => {
                    tracing::warn!("Error listing fonts from {}: {}", provider.name(), e);
                }
            }
        }
        
        let total = all_fonts.len();
        
        Ok(SearchResults {
            fonts: all_fonts,
            total,
            query: String::new(),
            providers_searched,
        })
    }
    
    /// Get detailed information about a specific font
    pub async fn get_font_details(&self, provider: &FontProvider, font_id: &str) -> Result<FontFamily> {
        for p in self.registry.providers() {
            if p.name() == provider.name() {
                return p.get_font_family(font_id).await;
            }
        }
        
        Err(anyhow::anyhow!("Provider not found: {:?}", provider))
    }
    
    /// Check health of all providers
    pub async fn health_check(&self) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        
        for provider in self.registry.providers() {
            let is_healthy = provider.health_check().await.unwrap_or(false);
            results.push((provider.name().to_string(), is_healthy));
        }
        
        results
    }
    
    /// Get statistics about available fonts
    pub async fn get_stats(&self) -> Result<FontStats> {
        let results = self.list_all().await?;
        
        let mut stats = FontStats::default();
        stats.total_fonts = results.total;
        stats.providers_count = results.providers_searched.len();
        stats.providers = results.providers_searched;
        
        // Count by category using parallel processing
        let category_counts: Vec<(Option<FontCategory>, usize)> = results.fonts
            .par_iter()
            .fold(
                || std::collections::HashMap::new(),
                |mut acc, font| {
                    *acc.entry(font.category.clone()).or_insert(0) += 1;
                    acc
                }
            )
            .reduce(
                || std::collections::HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        *a.entry(k).or_insert(0) += v;
                    }
                    a
                }
            )
            .into_iter()
            .collect();
        
        for (category, count) in category_counts {
            match category {
                Some(FontCategory::Serif) => stats.serif_count = count,
                Some(FontCategory::SansSerif) => stats.sans_serif_count = count,
                Some(FontCategory::Display) => stats.display_count = count,
                Some(FontCategory::Handwriting) => stats.handwriting_count = count,
                Some(FontCategory::Monospace) => stats.monospace_count = count,
                None => stats.uncategorized_count = count,
            }
        }
        
        Ok(stats)
    }
}

/// Font statistics
#[derive(Debug, Default, serde::Serialize)]
pub struct FontStats {
    pub total_fonts: usize,
    pub providers_count: usize,
    pub providers: Vec<String>,
    pub serif_count: usize,
    pub sans_serif_count: usize,
    pub display_count: usize,
    pub handwriting_count: usize,
    pub monospace_count: usize,
    pub uncategorized_count: usize,
}
