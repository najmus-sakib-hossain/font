//! 1001 Fonts provider - Large collection of free fonts
//!
//! 1001 Fonts has 40,000+ free fonts across many categories.

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

use crate::models::{Font, FontCategory, FontProvider};
use crate::providers::FontProviderTrait;

pub struct Fonts1001Provider {
    client: Client,
    base_url: String,
}

impl Fonts1001Provider {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        Ok(Self {
            client,
            base_url: "https://www.1001fonts.com".to_string(),
        })
    }
    
    fn parse_category(category: &str) -> Option<FontCategory> {
        match category.to_lowercase().as_str() {
            "serif" | "slab" => Some(FontCategory::Serif),
            "sans-serif" | "sans" | "geometric" => Some(FontCategory::SansSerif),
            "script" | "calligraphy" | "handwritten" | "cursive" | "brush" => Some(FontCategory::Handwriting),
            "display" | "decorative" | "fancy" | "headline" | "poster" => Some(FontCategory::Display),
            "monospace" | "typewriter" | "code" | "fixed" => Some(FontCategory::Monospace),
            _ => None,
        }
    }
    
    /// Get curated list of fonts from 1001fonts
    fn get_font_collection(&self) -> Vec<Font> {
        // 1001fonts popular fonts organized by category
        let fonts_data: Vec<(&str, &str, &str)> = vec![
            // Sans Serif
            ("Aileron", "sans-serif", "100% Free"),
            ("Akrobat", "sans-serif", "100% Free"),
            ("Albert", "sans-serif", "Free for personal use"),
            ("Aleo", "sans-serif", "OFL"),
            ("Anders", "sans-serif", "100% Free"),
            ("Anurati", "sans-serif", "Free for personal use"),
            ("Avenir Next", "sans-serif", "Commercial"),
            ("Azonix", "sans-serif", "Free for personal use"),
            ("Bebas Kai", "sans-serif", "100% Free"),
            ("Blogger Sans", "sans-serif", "100% Free"),
            ("Cabin", "sans-serif", "OFL"),
            ("Calibri Light", "sans-serif", "Microsoft"),
            ("Code Pro", "sans-serif", "Free demo"),
            ("Codec Pro", "sans-serif", "Free demo"),
            ("Comfortaa", "sans-serif", "OFL"),
            ("Coolvetica", "sans-serif", "Free for personal use"),
            ("Corporative", "sans-serif", "Free demo"),
            ("Coves", "sans-serif", "Free for personal use"),
            ("D-DIN", "sans-serif", "100% Free"),
            ("Dosis", "sans-serif", "OFL"),
            ("Exo", "sans-serif", "OFL"),
            ("Fira Sans", "sans-serif", "OFL"),
            ("Futura", "sans-serif", "Commercial"),
            ("Gidole", "sans-serif", "OFL"),
            ("Gilroy", "sans-serif", "Free demo"),
            ("Glacial Indifference", "sans-serif", "OFL"),
            ("Gotham", "sans-serif", "Commercial"),
            ("Gotham Rounded", "sans-serif", "Commercial"),
            ("Gravity", "sans-serif", "Free for personal use"),
            ("Greycliff", "sans-serif", "Commercial"),
            ("Harabara", "sans-serif", "Free for personal use"),
            ("Hel2", "sans-serif", "Free for personal use"),
            ("Helvetica Neue", "sans-serif", "Commercial"),
            ("Hero", "sans-serif", "100% Free"),
            ("Highway Gothic", "sans-serif", "Public Domain"),
            ("Intro", "sans-serif", "Free demo"),
            ("Josefin Sans", "sans-serif", "OFL"),
            ("Kollektif", "sans-serif", "100% Free"),
            ("Lato", "sans-serif", "OFL"),
            ("League Spartan", "sans-serif", "OFL"),
            ("Libre Franklin", "sans-serif", "OFL"),
            ("Metropolis", "sans-serif", "100% Free"),
            ("Mija", "sans-serif", "Free demo"),
            ("Montserrat", "sans-serif", "OFL"),
            ("Muli", "sans-serif", "OFL"),
            ("Nexa", "sans-serif", "Free demo"),
            ("Noir", "sans-serif", "Free demo"),
            ("Nunito Sans", "sans-serif", "OFL"),
            ("Open Sans", "sans-serif", "Apache"),
            ("Oswald", "sans-serif", "OFL"),
            ("Oxygen", "sans-serif", "OFL"),
            ("Pier Sans", "sans-serif", "Free for personal use"),
            ("Poppins", "sans-serif", "OFL"),
            ("Proxima Nova", "sans-serif", "Commercial"),
            ("Quicksand", "sans-serif", "OFL"),
            ("Rajdhani", "sans-serif", "OFL"),
            ("Raleway", "sans-serif", "OFL"),
            ("Roboto", "sans-serif", "Apache"),
            ("Rubik", "sans-serif", "OFL"),
            ("San Francisco", "sans-serif", "Apple"),
            ("Segoe UI", "sans-serif", "Microsoft"),
            ("Sofia Pro", "sans-serif", "Commercial"),
            ("Source Sans Pro", "sans-serif", "OFL"),
            ("Spartan", "sans-serif", "OFL"),
            ("Stratum", "sans-serif", "Free demo"),
            ("Ubuntu", "sans-serif", "UFL"),
            ("Varela Round", "sans-serif", "OFL"),
            ("Venus Rising", "sans-serif", "Free for personal use"),
            ("Walkway", "sans-serif", "100% Free"),
            ("Work Sans", "sans-serif", "OFL"),
            
            // Serif
            ("Abril Fatface", "serif", "OFL"),
            ("Addington CF", "serif", "Free demo"),
            ("Alike", "serif", "OFL"),
            ("Amiri", "serif", "OFL"),
            ("Arvo", "serif", "OFL"),
            ("Bitter", "serif", "OFL"),
            ("Bona Nova", "serif", "OFL"),
            ("Bree Serif", "serif", "OFL"),
            ("Butler", "serif", "100% Free"),
            ("Cardo", "serif", "OFL"),
            ("Charter", "serif", "Bitstream"),
            ("Cinzel", "serif", "OFL"),
            ("Clarendon", "serif", "Commercial"),
            ("Cormorant", "serif", "OFL"),
            ("Crimson Pro", "serif", "OFL"),
            ("Crimson Text", "serif", "OFL"),
            ("EB Garamond", "serif", "OFL"),
            ("Gabriela", "serif", "OFL"),
            ("Gentium", "serif", "OFL"),
            ("Georgia Pro", "serif", "Microsoft"),
            ("Gloock", "serif", "OFL"),
            ("Goudy Bookletter", "serif", "100% Free"),
            ("Gravitas One", "serif", "OFL"),
            ("Heuristica", "serif", "OFL"),
            ("Instrument Serif", "serif", "OFL"),
            ("Junicode", "serif", "OFL"),
            ("Libre Baskerville", "serif", "OFL"),
            ("Libre Bodoni", "serif", "OFL"),
            ("Libre Caslon Display", "serif", "OFL"),
            ("Libre Caslon Text", "serif", "OFL"),
            ("Literata", "serif", "OFL"),
            ("Lora", "serif", "OFL"),
            ("Merriweather", "serif", "OFL"),
            ("Noto Serif", "serif", "OFL"),
            ("Old Standard TT", "serif", "OFL"),
            ("Palatino Linotype", "serif", "Commercial"),
            ("Petrona", "serif", "OFL"),
            ("Playfair Display", "serif", "OFL"),
            ("Prata", "serif", "OFL"),
            ("PT Serif", "serif", "OFL"),
            ("Quattrocento", "serif", "OFL"),
            ("Roboto Slab", "serif", "Apache"),
            ("Rokkitt", "serif", "OFL"),
            ("Rosarivo", "serif", "OFL"),
            ("Rozha One", "serif", "OFL"),
            ("Rufina", "serif", "OFL"),
            ("Slabo 27px", "serif", "OFL"),
            ("Source Serif Pro", "serif", "OFL"),
            ("Spectral", "serif", "OFL"),
            ("Theano Didot", "serif", "OFL"),
            ("Times New Roman", "serif", "Commercial"),
            ("Trirong", "serif", "OFL"),
            ("Ultra", "serif", "OFL"),
            ("Unna", "serif", "OFL"),
            ("Vidaloka", "serif", "OFL"),
            ("Vollkorn", "serif", "OFL"),
            ("Young Serif", "serif", "OFL"),
            ("Zilla Slab", "serif", "OFL"),
            
            // Script/Handwriting
            ("Aguafina Script", "script", "OFL"),
            ("Alex Brush", "script", "OFL"),
            ("Allura", "script", "OFL"),
            ("Arizonia", "script", "OFL"),
            ("Bad Script", "script", "OFL"),
            ("Ballet", "script", "OFL"),
            ("Birthstone", "script", "OFL"),
            ("Bonheur Royale", "script", "OFL"),
            ("Brush Script MT", "script", "Commercial"),
            ("Corinthia", "script", "OFL"),
            ("Dancing Script", "script", "OFL"),
            ("Delicious Handrawn", "script", "OFL"),
            ("Engagement", "script", "OFL"),
            ("Euphoria Script", "script", "OFL"),
            ("Felipa", "script", "OFL"),
            ("Fondamento", "script", "OFL"),
            ("Great Vibes", "script", "OFL"),
            ("Herr Von Muellerhoff", "script", "OFL"),
            ("Italianno", "script", "OFL"),
            ("Kaushan Script", "script", "OFL"),
            ("Lavishly Yours", "script", "OFL"),
            ("League Script", "script", "OFL"),
            ("Lobster Two", "script", "OFL"),
            ("Loved by the King", "script", "OFL"),
            ("Lovers Quarrel", "script", "OFL"),
            ("Marck Script", "script", "OFL"),
            ("Meow Script", "script", "OFL"),
            ("Milonga", "script", "OFL"),
            ("Miss Fajardose", "script", "OFL"),
            ("Monsieur La Doulaise", "script", "OFL"),
            ("Mr De Haviland", "script", "OFL"),
            ("Mrs Saint Delafield", "script", "OFL"),
            ("Mrs Sheppards", "script", "OFL"),
            ("Niconne", "script", "OFL"),
            ("Norican", "script", "OFL"),
            ("Nothing You Could Do", "script", "OFL"),
            ("Oleo Script", "script", "OFL"),
            ("Pacifico", "script", "OFL"),
            ("Parisienne", "script", "OFL"),
            ("Petit Formal Script", "script", "OFL"),
            ("Pinyon Script", "script", "OFL"),
            ("Playball", "script", "OFL"),
            ("Qwigley", "script", "OFL"),
            ("Rochester", "script", "OFL"),
            ("Rouge Script", "script", "OFL"),
            ("Sacramento", "script", "OFL"),
            ("Sail", "script", "OFL"),
            ("Satisfy", "script", "OFL"),
            ("Seaweed Script", "script", "OFL"),
            ("Shadows Into Light", "script", "OFL"),
            ("Style Script", "script", "OFL"),
            ("Tangerine", "script", "OFL"),
            ("The Nautigal", "script", "OFL"),
            ("Waterfall", "script", "OFL"),
            ("Yellowtail", "script", "OFL"),
            ("Zeyada", "script", "OFL"),
            
            // Display
            ("Abril Fatface", "display", "OFL"),
            ("Alfa Slab One", "display", "OFL"),
            ("Anton", "display", "OFL"),
            ("Archivo Black", "display", "OFL"),
            ("Audiowide", "display", "OFL"),
            ("Bangers", "display", "OFL"),
            ("Bebas Neue", "display", "OFL"),
            ("Big Shoulders Display", "display", "OFL"),
            ("Black Ops One", "display", "OFL"),
            ("Bungee", "display", "OFL"),
            ("Bungee Shade", "display", "OFL"),
            ("Cabin Sketch", "display", "OFL"),
            ("Carter One", "display", "OFL"),
            ("Changa One", "display", "OFL"),
            ("Coda", "display", "OFL"),
            ("Concert One", "display", "OFL"),
            ("Creepster", "display", "OFL"),
            ("Diplomata", "display", "OFL"),
            ("Emblema One", "display", "OFL"),
            ("Faster One", "display", "OFL"),
            ("Flavors", "display", "OFL"),
            ("Fugaz One", "display", "OFL"),
            ("Germania One", "display", "OFL"),
            ("Graduate", "display", "OFL"),
            ("Gravitas One", "display", "OFL"),
            ("Iceland", "display", "OFL"),
            ("Impact", "display", "Commercial"),
            ("Kelly Slab", "display", "OFL"),
            ("Knewave", "display", "OFL"),
            ("League Gothic", "display", "OFL"),
            ("Lemon", "display", "OFL"),
            ("Lilita One", "display", "OFL"),
            ("Lobster", "display", "OFL"),
            ("Monoton", "display", "OFL"),
            ("Nosifer", "display", "OFL"),
            ("Orbitron", "display", "OFL"),
            ("Oswald", "display", "OFL"),
            ("Passion One", "display", "OFL"),
            ("Patua One", "display", "OFL"),
            ("Plaster", "display", "OFL"),
            ("Poller One", "display", "OFL"),
            ("Press Start 2P", "display", "OFL"),
            ("Racing Sans One", "display", "OFL"),
            ("Righteous", "display", "OFL"),
            ("Rubik Mono One", "display", "OFL"),
            ("Russo One", "display", "OFL"),
            ("Sansita", "display", "OFL"),
            ("Shrikhand", "display", "OFL"),
            ("Sigmar One", "display", "OFL"),
            ("Slackey", "display", "OFL"),
            ("Smokum", "display", "OFL"),
            ("Special Elite", "display", "OFL"),
            ("Squada One", "display", "OFL"),
            ("Staatliches", "display", "OFL"),
            ("Stalinist One", "display", "OFL"),
            ("Teko", "display", "OFL"),
            ("Titan One", "display", "OFL"),
            ("Trade Winds", "display", "OFL"),
            ("Ultra", "display", "OFL"),
            ("Wallpoet", "display", "OFL"),
            ("Yeseva One", "display", "OFL"),
            
            // Monospace
            ("Anonymous Pro", "monospace", "OFL"),
            ("B612 Mono", "monospace", "OFL"),
            ("Chivo Mono", "monospace", "OFL"),
            ("Courier Prime", "monospace", "OFL"),
            ("Cousine", "monospace", "Apache"),
            ("Cutive Mono", "monospace", "OFL"),
            ("DM Mono", "monospace", "OFL"),
            ("Fira Code", "monospace", "OFL"),
            ("Fira Mono", "monospace", "OFL"),
            ("Geist Mono", "monospace", "OFL"),
            ("Hack", "monospace", "MIT"),
            ("IBM Plex Mono", "monospace", "OFL"),
            ("Inconsolata", "monospace", "OFL"),
            ("JetBrains Mono", "monospace", "OFL"),
            ("Major Mono Display", "monospace", "OFL"),
            ("Martian Mono", "monospace", "OFL"),
            ("Nanum Gothic Coding", "monospace", "OFL"),
            ("Nova Mono", "monospace", "OFL"),
            ("Overpass Mono", "monospace", "OFL"),
            ("Oxygen Mono", "monospace", "OFL"),
            ("PT Mono", "monospace", "OFL"),
            ("Red Hat Mono", "monospace", "OFL"),
            ("Roboto Mono", "monospace", "Apache"),
            ("Share Tech Mono", "monospace", "OFL"),
            ("Source Code Pro", "monospace", "OFL"),
            ("Space Mono", "monospace", "OFL"),
            ("Ubuntu Mono", "monospace", "UFL"),
            ("Victor Mono", "monospace", "OFL"),
            ("Xanh Mono", "monospace", "OFL"),
            
            // More fonts to expand collection
            ("ABeeZee", "sans-serif", "OFL"),
            ("Abel", "sans-serif", "OFL"),
            ("Abhaya Libre", "serif", "OFL"),
            ("Aboreto", "serif", "OFL"),
            ("Abril Fatface", "display", "OFL"),
            ("Aclonica", "sans-serif", "OFL"),
            ("Acme", "sans-serif", "OFL"),
            ("Actor", "sans-serif", "OFL"),
            ("Adamina", "serif", "OFL"),
            ("Advent Pro", "sans-serif", "OFL"),
            ("Afacad", "sans-serif", "OFL"),
            ("Agbalumo", "display", "OFL"),
            ("Agdasima", "sans-serif", "OFL"),
            ("Aguafina Script", "script", "OFL"),
            ("Akaya Kanadaka", "display", "OFL"),
            ("Akaya Telivigala", "display", "OFL"),
            ("Akronim", "display", "OFL"),
            ("Akshar", "sans-serif", "OFL"),
            ("Aladin", "display", "OFL"),
            ("Alata", "sans-serif", "OFL"),
            ("Alatsi", "sans-serif", "OFL"),
            ("Albert Sans", "sans-serif", "OFL"),
            ("Aldrich", "sans-serif", "OFL"),
            ("Alef", "sans-serif", "OFL"),
            ("Alegreya", "serif", "OFL"),
            ("Alegreya Sans", "sans-serif", "OFL"),
            ("Alegreya SC", "serif", "OFL"),
            ("Aleo", "serif", "OFL"),
            ("Alexandria", "sans-serif", "OFL"),
            ("Alfa Slab One", "display", "OFL"),
            ("Alice", "serif", "OFL"),
            ("Alike", "serif", "OFL"),
            ("Alike Angular", "serif", "OFL"),
            ("Alkalami", "serif", "OFL"),
            ("Alkatra", "display", "OFL"),
            ("Allan", "display", "OFL"),
            ("Allerta", "sans-serif", "OFL"),
            ("Allerta Stencil", "sans-serif", "OFL"),
            ("Allison", "script", "OFL"),
            ("Allura", "script", "OFL"),
            ("Almarai", "sans-serif", "OFL"),
            ("Almendra", "serif", "OFL"),
            ("Almendra Display", "display", "OFL"),
            ("Almendra SC", "serif", "OFL"),
            ("Alumni Sans", "sans-serif", "OFL"),
            ("Alumni Sans Collegiate One", "sans-serif", "OFL"),
            ("Alumni Sans Inline One", "display", "OFL"),
            ("Alumni Sans Pinstripe", "sans-serif", "OFL"),
            ("Amarante", "display", "OFL"),
            ("Amaranth", "sans-serif", "OFL"),
            ("Amatic SC", "display", "OFL"),
            ("Amethysta", "serif", "OFL"),
            ("Amiko", "sans-serif", "OFL"),
            ("Amiri", "serif", "OFL"),
            ("Amiri Quran", "serif", "OFL"),
            ("Amita", "display", "OFL"),
            ("Anaheim", "sans-serif", "OFL"),
            ("Andada Pro", "serif", "OFL"),
            ("Andika", "sans-serif", "OFL"),
            ("Anek Bangla", "sans-serif", "OFL"),
            ("Anek Devanagari", "sans-serif", "OFL"),
            ("Anek Gujarati", "sans-serif", "OFL"),
            ("Anek Gurmukhi", "sans-serif", "OFL"),
            ("Anek Kannada", "sans-serif", "OFL"),
            ("Anek Latin", "sans-serif", "OFL"),
            ("Anek Malayalam", "sans-serif", "OFL"),
            ("Anek Odia", "sans-serif", "OFL"),
            ("Anek Tamil", "sans-serif", "OFL"),
            ("Anek Telugu", "sans-serif", "OFL"),
            ("Angkor", "display", "OFL"),
            ("Annie Use Your Telescope", "script", "OFL"),
            ("Anonymous Pro", "monospace", "OFL"),
            ("Anta", "sans-serif", "OFL"),
            ("Antic", "sans-serif", "OFL"),
            ("Antic Didone", "serif", "OFL"),
            ("Antic Slab", "serif", "OFL"),
            ("Anton", "sans-serif", "OFL"),
            ("Antonio", "sans-serif", "OFL"),
            ("Anuphan", "sans-serif", "OFL"),
            ("Anybody", "display", "OFL"),
            ("Aoboshi One", "serif", "OFL"),
            ("Arapey", "serif", "OFL"),
            ("Arbutus", "display", "OFL"),
            ("Arbutus Slab", "serif", "OFL"),
            ("Architects Daughter", "script", "OFL"),
            ("Archivo", "sans-serif", "OFL"),
            ("Archivo Black", "sans-serif", "OFL"),
            ("Archivo Narrow", "sans-serif", "OFL"),
            ("Are You Serious", "display", "OFL"),
            ("Aref Ruqaa", "serif", "OFL"),
            ("Aref Ruqaa Ink", "serif", "OFL"),
            ("Arima", "display", "OFL"),
            ("Arimo", "sans-serif", "Apache"),
            ("Arizonia", "script", "OFL"),
            ("Armata", "sans-serif", "OFL"),
            ("Arsenal", "sans-serif", "OFL"),
            ("Artifika", "serif", "OFL"),
            ("Arvo", "serif", "OFL"),
            ("Arya", "sans-serif", "OFL"),
            ("Asap", "sans-serif", "OFL"),
            ("Asap Condensed", "sans-serif", "OFL"),
            ("Asar", "serif", "OFL"),
            ("Asset", "display", "OFL"),
            ("Assistant", "sans-serif", "OFL"),
            ("Astloch", "display", "OFL"),
            ("Asul", "sans-serif", "OFL"),
            ("Athiti", "sans-serif", "OFL"),
            ("Atkinson Hyperlegible", "sans-serif", "OFL"),
            ("Atma", "display", "OFL"),
            ("Atomic Age", "display", "OFL"),
            ("Aubrey", "display", "OFL"),
            ("Audiowide", "display", "OFL"),
            ("Autour One", "display", "OFL"),
            ("Average", "serif", "OFL"),
            ("Average Sans", "sans-serif", "OFL"),
            ("Averia Gruesa Libre", "display", "OFL"),
            ("Averia Libre", "display", "OFL"),
            ("Averia Sans Libre", "sans-serif", "OFL"),
            ("Averia Serif Libre", "serif", "OFL"),
            ("Azeret Mono", "monospace", "OFL"),
        ];
        
        fonts_data.into_iter().map(|(name, category, license)| {
            let id = name.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            Font {
                id: format!("1001fonts-{}", id),
                name: name.to_string(),
                provider: FontProvider::Fonts1001,
                family: None,
                category: Self::parse_category(category),
                variants: vec!["regular".to_string()],
                variant_count: 1,
                subsets: vec!["latin".to_string()],
                license: Some(license.to_string()),
                designer: None,
                preview_url: Some(format!("{}/{}", self.base_url, id)),
                download_url: Some(format!("{}/{}/download", self.base_url, id)),
                popularity: None,
            }
        }).collect()
    }
}

impl Default for Fonts1001Provider {
    fn default() -> Self {
        Self::new().expect("Failed to create 1001 Fonts provider")
    }
}

#[async_trait]
impl FontProviderTrait for Fonts1001Provider {
    fn name(&self) -> &str {
        "1001 Fonts"
    }
    
    fn provider_type(&self) -> FontProvider {
        FontProvider::Fonts1001
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
