//! Font Squirrel provider - 100% free for commercial use fonts
//!
//! Font Squirrel curates high-quality free fonts licensed for commercial use.

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

use crate::models::{Font, FontCategory, FontProvider};
use crate::providers::FontProviderTrait;

pub struct FontSquirrelProvider {
    client: Client,
    base_url: String,
}

impl FontSquirrelProvider {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        Ok(Self {
            client,
            base_url: "https://www.fontsquirrel.com".to_string(),
        })
    }
    
    fn parse_category(category: &str) -> Option<FontCategory> {
        match category.to_lowercase().as_str() {
            "serif" => Some(FontCategory::Serif),
            "sans-serif" | "sans" => Some(FontCategory::SansSerif),
            "script" | "calligraphic" | "handdrawn" => Some(FontCategory::Handwriting),
            "display" | "decorative" | "retro" => Some(FontCategory::Display),
            "monospaced" | "typewriter" => Some(FontCategory::Monospace),
            _ => None,
        }
    }
    
    /// Get curated commercial-free fonts from Font Squirrel
    fn get_font_collection(&self) -> Vec<Font> {
        let fonts_data: Vec<(&str, &str)> = vec![
            // Sans Serif - 100% Free
            ("Aileron", "sans-serif"),
            ("Akrobat", "sans-serif"),
            ("Alef", "sans-serif"),
            ("Arimo", "sans-serif"),
            ("Archivo", "sans-serif"),
            ("Archivo Narrow", "sans-serif"),
            ("Archivo Black", "sans-serif"),
            ("Armata", "sans-serif"),
            ("Cabin", "sans-serif"),
            ("Carlito", "sans-serif"),
            ("Catamaran", "sans-serif"),
            ("Clear Sans", "sans-serif"),
            ("Comfortaa", "sans-serif"),
            ("Didact Gothic", "sans-serif"),
            ("Dosis", "sans-serif"),
            ("Encode Sans", "sans-serif"),
            ("Enriqueta", "sans-serif"),
            ("Exo", "sans-serif"),
            ("Fira Sans", "sans-serif"),
            ("Fontin Sans", "sans-serif"),
            ("Gidole", "sans-serif"),
            ("Glacial Indifference", "sans-serif"),
            ("Gothic A1", "sans-serif"),
            ("Gudea", "sans-serif"),
            ("Hind", "sans-serif"),
            ("Inter", "sans-serif"),
            ("Istok Web", "sans-serif"),
            ("Josefin Sans", "sans-serif"),
            ("Junction", "sans-serif"),
            ("Karla", "sans-serif"),
            ("Lato", "sans-serif"),
            ("League Spartan", "sans-serif"),
            ("Lexend Deca", "sans-serif"),
            ("Libre Franklin", "sans-serif"),
            ("Lora", "sans-serif"),
            ("Maven Pro", "sans-serif"),
            ("Metropolis", "sans-serif"),
            ("Montserrat", "sans-serif"),
            ("Muli", "sans-serif"),
            ("Nunito", "sans-serif"),
            ("Nunito Sans", "sans-serif"),
            ("Open Sans", "sans-serif"),
            ("Oswald", "sans-serif"),
            ("Overpass", "sans-serif"),
            ("Oxygen", "sans-serif"),
            ("Poppins", "sans-serif"),
            ("PT Sans", "sans-serif"),
            ("Questrial", "sans-serif"),
            ("Quicksand", "sans-serif"),
            ("Rajdhani", "sans-serif"),
            ("Raleway", "sans-serif"),
            ("Red Hat Display", "sans-serif"),
            ("Red Hat Text", "sans-serif"),
            ("Roboto", "sans-serif"),
            ("Rubik", "sans-serif"),
            ("Ruda", "sans-serif"),
            ("Saira", "sans-serif"),
            ("Signika", "sans-serif"),
            ("Source Sans Pro", "sans-serif"),
            ("Spartan", "sans-serif"),
            ("Titillium Web", "sans-serif"),
            ("Ubuntu", "sans-serif"),
            ("Varela", "sans-serif"),
            ("Varela Round", "sans-serif"),
            ("Work Sans", "sans-serif"),
            
            // Serif - 100% Free
            ("Aleo", "serif"),
            ("Alegreya", "serif"),
            ("Alike", "serif"),
            ("Amiri", "serif"),
            ("Arvo", "serif"),
            ("Bitter", "serif"),
            ("Bree Serif", "serif"),
            ("Butler", "serif"),
            ("Cardo", "serif"),
            ("Charter", "serif"),
            ("Cinzel", "serif"),
            ("Cormorant", "serif"),
            ("Cormorant Garamond", "serif"),
            ("Crimson Pro", "serif"),
            ("Crimson Text", "serif"),
            ("Crete Round", "serif"),
            ("Domine", "serif"),
            ("EB Garamond", "serif"),
            ("Gentium Basic", "serif"),
            ("Gentium Book Basic", "serif"),
            ("Goudy Bookletter 1911", "serif"),
            ("Heuristica", "serif"),
            ("Josefin Slab", "serif"),
            ("Jura", "serif"),
            ("Kreon", "serif"),
            ("Libre Baskerville", "serif"),
            ("Libre Bodoni", "serif"),
            ("Libre Caslon Text", "serif"),
            ("Literata", "serif"),
            ("Lora", "serif"),
            ("Lusitana", "serif"),
            ("Lustria", "serif"),
            ("Merriweather", "serif"),
            ("Neuton", "serif"),
            ("Noticia Text", "serif"),
            ("Noto Serif", "serif"),
            ("Old Standard TT", "serif"),
            ("Petrona", "serif"),
            ("Philosopher", "serif"),
            ("Playfair Display", "serif"),
            ("Podkova", "serif"),
            ("Prata", "serif"),
            ("PT Serif", "serif"),
            ("Quattrocento", "serif"),
            ("Roboto Slab", "serif"),
            ("Rokkitt", "serif"),
            ("Rosarivo", "serif"),
            ("Rufina", "serif"),
            ("Scope One", "serif"),
            ("Source Serif Pro", "serif"),
            ("Spectral", "serif"),
            ("Tinos", "serif"),
            ("Trirong", "serif"),
            ("Ultra", "serif"),
            ("Unna", "serif"),
            ("Vollkorn", "serif"),
            ("Zilla Slab", "serif"),
            
            // Script/Handwritten - 100% Free
            ("Alex Brush", "script"),
            ("Allison", "script"),
            ("Allura", "script"),
            ("Arizonia", "script"),
            ("Bad Script", "script"),
            ("Birthstone", "script"),
            ("Caveat", "script"),
            ("Courgette", "script"),
            ("Dancing Script", "script"),
            ("Engagement", "script"),
            ("Fondamento", "script"),
            ("Gloria Hallelujah", "script"),
            ("Gochi Hand", "script"),
            ("Grand Hotel", "script"),
            ("Great Vibes", "script"),
            ("Handlee", "script"),
            ("Homemade Apple", "script"),
            ("Indie Flower", "script"),
            ("Italianno", "script"),
            ("Kalam", "script"),
            ("Kaushan Script", "script"),
            ("Lobster Two", "script"),
            ("Marck Script", "script"),
            ("Neucha", "script"),
            ("Niconne", "script"),
            ("Pacifico", "script"),
            ("Parisienne", "script"),
            ("Patrick Hand", "script"),
            ("Permanent Marker", "script"),
            ("Pinyon Script", "script"),
            ("Playball", "script"),
            ("Rancho", "script"),
            ("Reenie Beanie", "script"),
            ("Rochester", "script"),
            ("Rock Salt", "script"),
            ("Rouge Script", "script"),
            ("Sacramento", "script"),
            ("Satisfy", "script"),
            ("Shadows Into Light", "script"),
            ("Tangerine", "script"),
            ("Yellowtail", "script"),
            
            // Display - 100% Free
            ("Abril Fatface", "display"),
            ("Alfa Slab One", "display"),
            ("Anton", "display"),
            ("Audiowide", "display"),
            ("Bangers", "display"),
            ("Bebas Neue", "display"),
            ("Bevan", "display"),
            ("Big Shoulders Display", "display"),
            ("Black Ops One", "display"),
            ("Bungee", "display"),
            ("Carter One", "display"),
            ("Changa One", "display"),
            ("Comfortaa", "display"),
            ("Concert One", "display"),
            ("Creepster", "display"),
            ("Fredoka One", "display"),
            ("Germania One", "display"),
            ("Graduate", "display"),
            ("Iceland", "display"),
            ("Kelly Slab", "display"),
            ("Knewave", "display"),
            ("League Gothic", "display"),
            ("Lilita One", "display"),
            ("Lobster", "display"),
            ("Luckiest Guy", "display"),
            ("Monoton", "display"),
            ("Nixie One", "display"),
            ("Nosifer", "display"),
            ("Oleo Script", "display"),
            ("Orbitron", "display"),
            ("Passion One", "display"),
            ("Patua One", "display"),
            ("Plaster", "display"),
            ("Press Start 2P", "display"),
            ("Racing Sans One", "display"),
            ("Righteous", "display"),
            ("Russo One", "display"),
            ("Shrikhand", "display"),
            ("Special Elite", "display"),
            ("Staatliches", "display"),
            ("Stalinist One", "display"),
            ("Teko", "display"),
            ("Titan One", "display"),
            ("Trade Winds", "display"),
            ("Wallpoet", "display"),
            ("Yeseva One", "display"),
            
            // Monospace - 100% Free
            ("Anonymous Pro", "monospaced"),
            ("Azeret Mono", "monospaced"),
            ("B612 Mono", "monospaced"),
            ("Courier Prime", "monospaced"),
            ("Cousine", "monospaced"),
            ("Cutive Mono", "monospaced"),
            ("DM Mono", "monospaced"),
            ("Fantasque Sans Mono", "monospaced"),
            ("Fira Code", "monospaced"),
            ("Fira Mono", "monospaced"),
            ("Hack", "monospaced"),
            ("IBM Plex Mono", "monospaced"),
            ("Inconsolata", "monospaced"),
            ("Input Mono", "monospaced"),
            ("JetBrains Mono", "monospaced"),
            ("Liberation Mono", "monospaced"),
            ("Major Mono Display", "monospaced"),
            ("Nanum Gothic Coding", "monospaced"),
            ("Nova Mono", "monospaced"),
            ("Overpass Mono", "monospaced"),
            ("Oxygen Mono", "monospaced"),
            ("PT Mono", "monospaced"),
            ("Red Hat Mono", "monospaced"),
            ("Roboto Mono", "monospaced"),
            ("Share Tech Mono", "monospaced"),
            ("Source Code Pro", "monospaced"),
            ("Space Mono", "monospaced"),
            ("Ubuntu Mono", "monospaced"),
            ("Victor Mono", "monospaced"),
            
            // Additional fonts
            ("Abhaya Libre", "serif"),
            ("Abel", "sans-serif"),
            ("Aboreto", "sans-serif"),
            ("Aclonica", "sans-serif"),
            ("Acme", "sans-serif"),
            ("Actor", "sans-serif"),
            ("Adamina", "serif"),
            ("Advent Pro", "sans-serif"),
            ("Afacad", "sans-serif"),
            ("Agdasima", "sans-serif"),
            ("Aguafina Script", "script"),
            ("Akronim", "display"),
            ("Akshar", "sans-serif"),
            ("Aladin", "display"),
            ("Alata", "sans-serif"),
            ("Alatsi", "sans-serif"),
            ("Albert Sans", "sans-serif"),
            ("Aldrich", "sans-serif"),
            ("Alegreya Sans", "sans-serif"),
            ("Alegreya SC", "serif"),
            ("Alexandria", "sans-serif"),
            ("Alice", "serif"),
            ("Alike Angular", "serif"),
            ("Alkalami", "serif"),
            ("Alkatra", "display"),
            ("Allan", "display"),
            ("Allerta", "sans-serif"),
            ("Allerta Stencil", "sans-serif"),
            ("Almendra", "serif"),
            ("Almendra Display", "display"),
            ("Almendra SC", "serif"),
            ("Alumni Sans", "sans-serif"),
            ("Alumni Sans Collegiate One", "sans-serif"),
            ("Amarante", "display"),
            ("Amaranth", "sans-serif"),
            ("Amatic SC", "display"),
            ("Amethysta", "serif"),
            ("Amiko", "sans-serif"),
            ("Amita", "display"),
            ("Anaheim", "sans-serif"),
            ("Andada Pro", "serif"),
            ("Andika", "sans-serif"),
            ("Angkor", "display"),
            ("Annie Use Your Telescope", "script"),
            ("Anta", "sans-serif"),
            ("Antic", "sans-serif"),
            ("Antic Didone", "serif"),
            ("Antic Slab", "serif"),
            ("Antonio", "sans-serif"),
            ("Anuphan", "sans-serif"),
            ("Anybody", "display"),
            ("Arapey", "serif"),
            ("Arbutus", "display"),
            ("Arbutus Slab", "serif"),
            ("Architects Daughter", "script"),
            ("Are You Serious", "display"),
            ("Aref Ruqaa", "serif"),
            ("Aref Ruqaa Ink", "serif"),
            ("Arima", "display"),
            ("Arsenal", "sans-serif"),
            ("Artifika", "serif"),
            ("Arya", "sans-serif"),
            ("Asap", "sans-serif"),
            ("Asap Condensed", "sans-serif"),
            ("Asar", "serif"),
            ("Asset", "display"),
            ("Assistant", "sans-serif"),
            ("Astloch", "display"),
            ("Asul", "sans-serif"),
            ("Athiti", "sans-serif"),
            ("Atkinson Hyperlegible", "sans-serif"),
            ("Atma", "display"),
            ("Atomic Age", "display"),
            ("Aubrey", "display"),
            ("Autour One", "display"),
            ("Average", "serif"),
            ("Average Sans", "sans-serif"),
            ("Averia Gruesa Libre", "display"),
            ("Averia Libre", "display"),
            ("Averia Sans Libre", "sans-serif"),
            ("Averia Serif Libre", "serif"),
        ];
        
        fonts_data.into_iter().map(|(name, category)| {
            let id = name.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            Font {
                id: format!("fontsquirrel-{}", id),
                name: name.to_string(),
                provider: FontProvider::FontSquirrel,
                family: None,
                category: Self::parse_category(category),
                variants: vec!["regular".to_string()],
                variant_count: 1,
                subsets: vec!["latin".to_string()],
                license: Some("100% Free".to_string()),
                designer: None,
                preview_url: Some(format!("{}/fonts/{}", self.base_url, id)),
                download_url: Some(format!("{}/fonts/download/{}", self.base_url, id)),
                popularity: None,
            }
        }).collect()
    }
}

impl Default for FontSquirrelProvider {
    fn default() -> Self {
        Self::new().expect("Failed to create FontSquirrel provider")
    }
}

#[async_trait]
impl FontProviderTrait for FontSquirrelProvider {
    fn name(&self) -> &str {
        "Font Squirrel"
    }
    
    fn provider_type(&self) -> FontProvider {
        FontProvider::FontSquirrel
    }
    
    async fn list_fonts(&self) -> Result<Vec<Font>> {
        Ok(self.get_font_collection())
    }
    
    async fn search(&self, query: &str) -> Result<Vec<Font>> {
        let all_fonts = self.get_font_collection();
        let query_lower = query.to_lowercase();
        
        Ok(all_fonts
            .into_iter()
            .filter(|font| {
                font.name.to_lowercase().contains(&query_lower) ||
                font.category.as_ref().map(|c| format!("{:?}", c).to_lowercase().contains(&query_lower)).unwrap_or(false)
            })
            .collect())
    }
    
    async fn get_font(&self, id: &str) -> Result<Option<Font>> {
        let fonts = self.get_font_collection();
        Ok(fonts.into_iter().find(|f| f.id == id))
    }
    
    async fn download_font(&self, font: &Font, output_dir: &std::path::Path) -> Result<std::path::PathBuf> {
        let download_url = font.download_url.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No download URL for font"))?;
        
        let response = self.client.get(download_url).send().await?;
        let bytes = response.bytes().await?;
        
        let file_path = output_dir.join(format!("{}.zip", font.id));
        std::fs::write(&file_path, &bytes)?;
        
        Ok(file_path)
    }
    
    async fn health_check(&self) -> bool {
        self.client.get(&self.base_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
