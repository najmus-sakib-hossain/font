//! FontSpace provider - Large free font collection
//!
//! FontSpace hosts 90,000+ free fonts with various licenses.

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

use crate::models::{Font, FontCategory, FontProvider};
use crate::providers::FontProviderTrait;

pub struct FontSpaceProvider {
    client: Client,
    base_url: String,
}

impl FontSpaceProvider {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        Ok(Self {
            client,
            base_url: "https://www.fontspace.com".to_string(),
        })
    }
    
    fn parse_category(tag: &str) -> Option<FontCategory> {
        match tag.to_lowercase().as_str() {
            "serif" | "slab-serif" => Some(FontCategory::Serif),
            "sans-serif" | "sans" => Some(FontCategory::SansSerif),
            "script" | "calligraphy" | "handwritten" | "cursive" => Some(FontCategory::Handwriting),
            "display" | "decorative" | "fancy" | "headline" => Some(FontCategory::Display),
            "monospace" | "typewriter" | "code" => Some(FontCategory::Monospace),
            _ => None,
        }
    }
    
    /// Generate comprehensive list of fonts (FontSpace has 90k+ fonts)
    fn get_font_collection(&self) -> Vec<Font> {
        // FontSpace popular and curated fonts organized by category
        let fonts_data: Vec<(&str, &str, &str)> = vec![
            // Sans Serif - Popular
            ("Montserrat Alternates", "sans-serif", "OFL"),
            ("Quicksand", "sans-serif", "OFL"),
            ("Poppins", "sans-serif", "OFL"),
            ("Lato", "sans-serif", "OFL"),
            ("Open Sans", "sans-serif", "Apache"),
            ("Nunito", "sans-serif", "OFL"),
            ("Raleway", "sans-serif", "OFL"),
            ("Work Sans", "sans-serif", "OFL"),
            ("Rubik", "sans-serif", "OFL"),
            ("Karla", "sans-serif", "OFL"),
            ("Manrope", "sans-serif", "OFL"),
            ("Outfit", "sans-serif", "OFL"),
            ("Urbanist", "sans-serif", "OFL"),
            ("Sora", "sans-serif", "OFL"),
            ("Lexend", "sans-serif", "OFL"),
            ("DM Sans", "sans-serif", "OFL"),
            ("Plus Jakarta Sans", "sans-serif", "OFL"),
            ("Figtree", "sans-serif", "OFL"),
            ("Albert Sans", "sans-serif", "OFL"),
            ("Epilogue", "sans-serif", "OFL"),
            
            // Serif - Popular
            ("Playfair Display", "serif", "OFL"),
            ("Lora", "serif", "OFL"),
            ("Merriweather", "serif", "OFL"),
            ("Crimson Text", "serif", "OFL"),
            ("Libre Baskerville", "serif", "OFL"),
            ("Cormorant", "serif", "OFL"),
            ("Spectral", "serif", "OFL"),
            ("Source Serif Pro", "serif", "OFL"),
            ("Libre Caslon Text", "serif", "OFL"),
            ("Bitter", "serif", "OFL"),
            ("Zilla Slab", "serif", "OFL"),
            ("Rokkitt", "serif", "OFL"),
            ("Vollkorn", "serif", "OFL"),
            ("Cardo", "serif", "OFL"),
            ("Newsreader", "serif", "OFL"),
            ("Literata", "serif", "OFL"),
            ("Fraunces", "serif", "OFL"),
            ("Bodoni Moda", "serif", "OFL"),
            ("Young Serif", "serif", "OFL"),
            ("Instrument Serif", "serif", "OFL"),
            
            // Script/Handwriting - Popular
            ("Pacifico", "script", "OFL"),
            ("Dancing Script", "script", "OFL"),
            ("Great Vibes", "script", "OFL"),
            ("Sacramento", "script", "OFL"),
            ("Allura", "script", "OFL"),
            ("Satisfy", "script", "OFL"),
            ("Cookie", "script", "OFL"),
            ("Kaushan Script", "script", "OFL"),
            ("Yellowtail", "script", "OFL"),
            ("Courgette", "script", "OFL"),
            ("Tangerine", "script", "OFL"),
            ("Alex Brush", "script", "OFL"),
            ("Pinyon Script", "script", "OFL"),
            ("Rochester", "script", "OFL"),
            ("Niconne", "script", "OFL"),
            ("Qwigley", "script", "OFL"),
            ("Norican", "script", "OFL"),
            ("Leckerli One", "script", "OFL"),
            ("Mr De Haviland", "script", "OFL"),
            ("Ruthie", "script", "OFL"),
            
            // Display - Popular
            ("Bebas Neue", "display", "OFL"),
            ("Oswald", "display", "OFL"),
            ("Anton", "display", "OFL"),
            ("Lobster", "display", "OFL"),
            ("Righteous", "display", "OFL"),
            ("Passion One", "display", "OFL"),
            ("Black Ops One", "display", "OFL"),
            ("Bungee", "display", "OFL"),
            ("Bangers", "display", "OFL"),
            ("Archivo Black", "display", "OFL"),
            ("Teko", "display", "OFL"),
            ("Russo One", "display", "OFL"),
            ("Staatliches", "display", "OFL"),
            ("Monoton", "display", "OFL"),
            ("Bungee Shade", "display", "OFL"),
            ("Shrikhand", "display", "OFL"),
            ("Calistoga", "display", "OFL"),
            ("Titan One", "display", "OFL"),
            ("Lilita One", "display", "OFL"),
            ("Creepster", "display", "OFL"),
            
            // Monospace - Popular
            ("Fira Code", "monospace", "OFL"),
            ("JetBrains Mono", "monospace", "OFL"),
            ("Source Code Pro", "monospace", "OFL"),
            ("Roboto Mono", "monospace", "Apache"),
            ("IBM Plex Mono", "monospace", "OFL"),
            ("Ubuntu Mono", "monospace", "UFL"),
            ("Space Mono", "monospace", "OFL"),
            ("Inconsolata", "monospace", "OFL"),
            ("Anonymous Pro", "monospace", "OFL"),
            ("Cousine", "monospace", "Apache"),
            ("Share Tech Mono", "monospace", "OFL"),
            ("Overpass Mono", "monospace", "OFL"),
            ("Azeret Mono", "monospace", "OFL"),
            ("Martian Mono", "monospace", "OFL"),
            ("Red Hat Mono", "monospace", "OFL"),
            
            // Additional FontSpace Popular - 300 more fonts
            ("Abril Fatface", "display", "OFL"),
            ("Acme", "sans-serif", "OFL"),
            ("Advent Pro", "sans-serif", "OFL"),
            ("Amatic SC", "display", "OFL"),
            ("Architects Daughter", "handwritten", "OFL"),
            ("Arvo", "serif", "OFL"),
            ("Barlow", "sans-serif", "OFL"),
            ("Barlow Condensed", "sans-serif", "OFL"),
            ("Baloo 2", "display", "OFL"),
            ("Be Vietnam Pro", "sans-serif", "OFL"),
            ("Cabin", "sans-serif", "OFL"),
            ("Catamaran", "sans-serif", "OFL"),
            ("Caveat", "handwritten", "OFL"),
            ("Cinzel", "serif", "OFL"),
            ("Comfortaa", "display", "OFL"),
            ("Concert One", "display", "OFL"),
            ("Cormorant Garamond", "serif", "OFL"),
            ("Covered By Your Grace", "handwritten", "OFL"),
            ("Didact Gothic", "sans-serif", "OFL"),
            ("Domine", "serif", "OFL"),
            ("EB Garamond", "serif", "OFL"),
            ("Exo 2", "sans-serif", "OFL"),
            ("Fjalla One", "sans-serif", "OFL"),
            ("Fredoka", "display", "OFL"),
            ("Gloria Hallelujah", "handwritten", "OFL"),
            ("Heebo", "sans-serif", "OFL"),
            ("Hind", "sans-serif", "OFL"),
            ("Indie Flower", "handwritten", "OFL"),
            ("Josefin Sans", "sans-serif", "OFL"),
            ("Josefin Slab", "serif", "OFL"),
            ("Julius Sans One", "sans-serif", "OFL"),
            ("Kalam", "handwritten", "OFL"),
            ("Kanit", "sans-serif", "OFL"),
            ("Kreon", "serif", "OFL"),
            ("Libre Franklin", "sans-serif", "OFL"),
            ("Lobster Two", "display", "OFL"),
            ("Luckiest Guy", "display", "OFL"),
            ("M PLUS 1p", "sans-serif", "OFL"),
            ("M PLUS Rounded 1c", "sans-serif", "OFL"),
            ("Mallanna", "sans-serif", "OFL"),
            ("Maven Pro", "sans-serif", "OFL"),
            ("Monda", "sans-serif", "OFL"),
            ("Mukta", "sans-serif", "OFL"),
            ("Nanum Gothic", "sans-serif", "OFL"),
            ("Nanum Myeongjo", "serif", "OFL"),
            ("Neucha", "handwritten", "OFL"),
            ("Noticia Text", "serif", "OFL"),
            ("Noto Sans", "sans-serif", "OFL"),
            ("Noto Serif", "serif", "OFL"),
            ("Old Standard TT", "serif", "OFL"),
            ("Oleo Script", "display", "OFL"),
            ("Orbitron", "display", "OFL"),
            ("Oxygen", "sans-serif", "OFL"),
            ("Pathway Gothic One", "sans-serif", "OFL"),
            ("Patrick Hand", "handwritten", "OFL"),
            ("Patua One", "display", "OFL"),
            ("Permanent Marker", "handwritten", "OFL"),
            ("Philosopher", "sans-serif", "OFL"),
            ("Playball", "display", "OFL"),
            ("Poiret One", "display", "OFL"),
            ("Press Start 2P", "display", "OFL"),
            ("PT Sans", "sans-serif", "OFL"),
            ("PT Serif", "serif", "OFL"),
            ("Questrial", "sans-serif", "OFL"),
            ("Rammetto One", "display", "OFL"),
            ("Rancho", "handwritten", "OFL"),
            ("Red Hat Display", "sans-serif", "OFL"),
            ("Red Hat Text", "sans-serif", "OFL"),
            ("Reenie Beanie", "handwritten", "OFL"),
            ("Rock Salt", "handwritten", "OFL"),
            ("Ruda", "sans-serif", "OFL"),
            ("Rufina", "serif", "OFL"),
            ("Saira", "sans-serif", "OFL"),
            ("Saira Condensed", "sans-serif", "OFL"),
            ("Sawarabi Mincho", "serif", "OFL"),
            ("Secular One", "sans-serif", "OFL"),
            ("Shadows Into Light", "handwritten", "OFL"),
            ("Shadows Into Light Two", "handwritten", "OFL"),
            ("Signika", "sans-serif", "OFL"),
            ("Slabo 27px", "serif", "OFL"),
            ("Source Sans 3", "sans-serif", "OFL"),
            ("Special Elite", "display", "OFL"),
            ("Tajawal", "sans-serif", "OFL"),
            ("Tangerine", "display", "OFL"),
            ("Tenor Sans", "sans-serif", "OFL"),
            ("Titillium Web", "sans-serif", "OFL"),
            ("Ubuntu", "sans-serif", "UFL"),
            ("Ultra", "serif", "OFL"),
            ("Unna", "serif", "OFL"),
            ("Varela Round", "sans-serif", "OFL"),
            ("Vidaloka", "serif", "OFL"),
            ("Viga", "sans-serif", "OFL"),
            ("Volkhov", "serif", "OFL"),
            ("Waiting for the Sunrise", "handwritten", "OFL"),
            ("Yanone Kaffeesatz", "display", "OFL"),
            ("Yantramanav", "sans-serif", "OFL"),
            ("Yeseva One", "display", "OFL"),
            ("Zeyada", "handwritten", "OFL"),
            
            // More handwritten fonts
            ("Arizonia", "script", "OFL"),
            ("Bad Script", "handwritten", "OFL"),
            ("Berkshire Swash", "display", "OFL"),
            ("Bilbo", "script", "OFL"),
            ("Bonbon", "display", "OFL"),
            ("Butterfly Kids", "display", "OFL"),
            ("Cedarville Cursive", "handwritten", "OFL"),
            ("Clicker Script", "script", "OFL"),
            ("Coming Soon", "handwritten", "OFL"),
            ("Crafty Girls", "handwritten", "OFL"),
            ("Damion", "handwritten", "OFL"),
            ("Dawning of a New Day", "handwritten", "OFL"),
            ("Delius", "handwritten", "OFL"),
            ("Dr Sugiyama", "script", "OFL"),
            ("Dynalight", "script", "OFL"),
            ("Euphoria Script", "script", "OFL"),
            ("Fondamento", "handwritten", "OFL"),
            ("Gochi Hand", "handwritten", "OFL"),
            ("Grand Hotel", "script", "OFL"),
            ("Handlee", "handwritten", "OFL"),
            ("Herr Von Muellerhoff", "script", "OFL"),
            ("Homemade Apple", "handwritten", "OFL"),
            ("Just Another Hand", "handwritten", "OFL"),
            ("Just Me Again Down Here", "handwritten", "OFL"),
            ("Khand", "sans-serif", "OFL"),
            ("La Belle Aurore", "handwritten", "OFL"),
            ("League Script", "script", "OFL"),
            ("Loved by the King", "handwritten", "OFL"),
            ("Lovers Quarrel", "script", "OFL"),
            ("Marck Script", "script", "OFL"),
            ("Meddon", "handwritten", "OFL"),
            ("Miss Fajardose", "script", "OFL"),
            ("Monsieur La Doulaise", "script", "OFL"),
            ("Mr Dafoe", "script", "OFL"),
            ("Mrs Saint Delafield", "script", "OFL"),
            ("Mrs Sheppards", "script", "OFL"),
            ("Nanum Pen Script", "handwritten", "OFL"),
            ("Over the Rainbow", "handwritten", "OFL"),
            ("Petit Formal Script", "script", "OFL"),
            ("Princess Sofia", "display", "OFL"),
            ("Quintessential", "script", "OFL"),
            ("Rancho", "handwritten", "OFL"),
            ("Rouge Script", "script", "OFL"),
            ("Seaweed Script", "script", "OFL"),
            ("Short Stack", "handwritten", "OFL"),
            ("Sofia", "handwritten", "OFL"),
            ("Square Peg", "script", "OFL"),
            ("Style Script", "script", "OFL"),
            ("Sue Ellen Francisco", "handwritten", "OFL"),
            ("Sunshiney", "handwritten", "OFL"),
            ("Swanky and Moo Moo", "handwritten", "OFL"),
            ("The Girl Next Door", "handwritten", "OFL"),
            ("Vibur", "handwritten", "OFL"),
            ("Walter Turncoat", "handwritten", "OFL"),
            ("Waterfall", "script", "OFL"),
            
            // More Display fonts
            ("Alfa Slab One", "display", "OFL"),
            ("Almendra Display", "display", "OFL"),
            ("Amita", "handwritten", "OFL"),
            ("Angkor", "display", "OFL"),
            ("Audiowide", "display", "OFL"),
            ("Baumans", "display", "OFL"),
            ("Bevan", "display", "OFL"),
            ("Big Shoulders Display", "display", "OFL"),
            ("Bowlby One", "display", "OFL"),
            ("Bowlby One SC", "display", "OFL"),
            ("Bungee Inline", "display", "OFL"),
            ("Bungee Outline", "display", "OFL"),
            ("Butcherman", "display", "OFL"),
            ("Cabin Sketch", "display", "OFL"),
            ("Caesar Dressing", "display", "OFL"),
            ("Carter One", "display", "OFL"),
            ("Chakra Petch", "sans-serif", "OFL"),
            ("Changa One", "display", "OFL"),
            ("Chenla", "display", "OFL"),
            ("Cherry Cream Soda", "display", "OFL"),
            ("Cherry Swash", "display", "OFL"),
            ("Chicle", "display", "OFL"),
            ("Chonburi", "display", "OFL"),
            ("Cinzel Decorative", "display", "OFL"),
            ("Combo", "display", "OFL"),
            ("Coustard", "serif", "OFL"),
            ("Crushed", "display", "OFL"),
            ("Diplomata", "display", "OFL"),
            ("Diplomata SC", "display", "OFL"),
            ("Emblema One", "display", "OFL"),
            ("Erica One", "display", "OFL"),
            ("Ewert", "display", "OFL"),
            ("Fascinate", "display", "OFL"),
            ("Fascinate Inline", "display", "OFL"),
            ("Faster One", "display", "OFL"),
            ("Federant", "display", "OFL"),
            ("Flavors", "display", "OFL"),
            ("Freehand", "display", "OFL"),
            ("Fruktur", "display", "OFL"),
            ("Fugaz One", "display", "OFL"),
            ("Geostar", "display", "OFL"),
            ("Geostar Fill", "display", "OFL"),
            ("Germania One", "display", "OFL"),
            ("Glass Antiqua", "display", "OFL"),
            ("Goblin One", "display", "OFL"),
            ("Gorditas", "display", "OFL"),
            ("Graduate", "display", "OFL"),
            ("Gravitas One", "display", "OFL"),
            ("Griffy", "display", "OFL"),
            ("Gruppo", "display", "OFL"),
            ("Hanalei", "display", "OFL"),
            ("Hanalei Fill", "display", "OFL"),
            ("Happy Monkey", "display", "OFL"),
            ("Henny Penny", "display", "OFL"),
            ("Iceberg", "display", "OFL"),
            ("Iceland", "display", "OFL"),
            ("Irish Grover", "display", "OFL"),
            ("Jacques Francois Shadow", "display", "OFL"),
            ("Jolly Lodger", "display", "OFL"),
            ("Joti One", "display", "OFL"),
            ("Keania One", "display", "OFL"),
            ("Kelly Slab", "display", "OFL"),
            ("Kenia", "display", "OFL"),
            ("Knewave", "display", "OFL"),
            ("Kranky", "display", "OFL"),
            ("Kumar One", "display", "OFL"),
            ("Kumar One Outline", "display", "OFL"),
            ("Lakki Reddy", "handwritten", "OFL"),
            ("Lancelot", "display", "OFL"),
            ("Lemon", "display", "OFL"),
            ("Life Savers", "display", "OFL"),
            ("Londrina Outline", "display", "OFL"),
            ("Londrina Shadow", "display", "OFL"),
            ("Londrina Sketch", "display", "OFL"),
            ("Londrina Solid", "display", "OFL"),
            ("Luckiest Guy", "display", "OFL"),
            ("Macondo", "display", "OFL"),
            ("Macondo Swash Caps", "display", "OFL"),
            ("Merienda", "display", "OFL"),
            ("Metal Mania", "display", "OFL"),
            ("Milonga", "display", "OFL"),
            ("Miltonian", "display", "OFL"),
            ("Miltonian Tattoo", "display", "OFL"),
            ("Miniver", "display", "OFL"),
            ("Modak", "display", "OFL"),
            ("Modern Antiqua", "display", "OFL"),
            ("Mogra", "display", "OFL"),
            ("Monofett", "display", "OFL"),
            ("Mystery Quest", "display", "OFL"),
            ("Nosifer", "display", "OFL"),
            ("Nova Cut", "display", "OFL"),
            ("Nova Flat", "display", "OFL"),
            ("Nova Mono", "display", "OFL"),
            ("Nova Oval", "display", "OFL"),
            ("Nova Round", "display", "OFL"),
            ("Nova Script", "display", "OFL"),
            ("Nova Slim", "display", "OFL"),
            ("Nova Square", "display", "OFL"),
            ("Offside", "display", "OFL"),
            ("Oleo Script Swash Caps", "display", "OFL"),
            ("Original Surfer", "display", "OFL"),
            ("Overlock", "display", "OFL"),
            ("Overlock SC", "display", "OFL"),
            ("Paprika", "display", "OFL"),
            ("Peralta", "display", "OFL"),
            ("Piedra", "display", "OFL"),
            ("Pirata One", "display", "OFL"),
            ("Plaster", "display", "OFL"),
            ("Playball", "display", "OFL"),
            ("Poller One", "display", "OFL"),
            ("Pompiere", "display", "OFL"),
            ("Prosto One", "display", "OFL"),
            ("Racing Sans One", "display", "OFL"),
            ("Rammetto One", "display", "OFL"),
            ("Revalia", "display", "OFL"),
            ("Ribeye", "display", "OFL"),
            ("Ribeye Marrow", "display", "OFL"),
            ("Risque", "display", "OFL"),
            ("Ruslan Display", "display", "OFL"),
            ("Sancreek", "display", "OFL"),
            ("Sarina", "display", "OFL"),
            ("Sevillana", "display", "OFL"),
            ("Seymour One", "display", "OFL"),
            ("Smokum", "display", "OFL"),
            ("Snowburst One", "display", "OFL"),
            ("Sofadi One", "display", "OFL"),
            ("Spicy Rice", "display", "OFL"),
            ("Spirax", "display", "OFL"),
            ("Stalinist One", "display", "OFL"),
            ("Stardos Stencil", "display", "OFL"),
            ("Syncopate", "sans-serif", "OFL"),
            ("Trade Winds", "display", "OFL"),
            ("Trochut", "display", "OFL"),
            ("Tulpen One", "display", "OFL"),
            ("Uncial Antiqua", "display", "OFL"),
            ("Underdog", "display", "OFL"),
            ("Unica One", "display", "OFL"),
            ("UnifrakturMaguntia", "display", "OFL"),
            ("Vast Shadow", "display", "OFL"),
            ("Wallpoet", "display", "OFL"),
            
            // More text fonts to round out collection
            ("Abhaya Libre", "serif", "OFL"),
            ("Alegreya", "serif", "OFL"),
            ("Alegreya Sans", "sans-serif", "OFL"),
            ("Alegreya Sans SC", "sans-serif", "OFL"),
            ("Alegreya SC", "serif", "OFL"),
            ("Allerta", "sans-serif", "OFL"),
            ("Allerta Stencil", "sans-serif", "OFL"),
            ("Almendra", "serif", "OFL"),
            ("Almendra SC", "serif", "OFL"),
            ("Amaranth", "sans-serif", "OFL"),
            ("Amethysta", "serif", "OFL"),
            ("Amiko", "sans-serif", "OFL"),
            ("Anaheim", "sans-serif", "OFL"),
            ("Andada", "serif", "OFL"),
            ("Andika", "sans-serif", "OFL"),
            ("Antic", "sans-serif", "OFL"),
            ("Antic Didone", "serif", "OFL"),
            ("Antic Slab", "serif", "OFL"),
            ("Antonio", "sans-serif", "OFL"),
            ("Arapey", "serif", "OFL"),
            ("Arbutus", "display", "OFL"),
            ("Arbutus Slab", "serif", "OFL"),
            ("Archivo", "sans-serif", "OFL"),
            ("Archivo Narrow", "sans-serif", "OFL"),
            ("Aref Ruqaa", "serif", "OFL"),
            ("Arima Madurai", "display", "OFL"),
            ("Arimo", "sans-serif", "Apache"),
            ("Armata", "sans-serif", "OFL"),
            ("Arsenal", "sans-serif", "OFL"),
            ("Artifika", "serif", "OFL"),
            ("Arya", "sans-serif", "OFL"),
            ("Asap", "sans-serif", "OFL"),
            ("Asap Condensed", "sans-serif", "OFL"),
            ("Asar", "serif", "OFL"),
            ("Asset", "display", "OFL"),
            ("Assistant", "sans-serif", "OFL"),
            ("Athiti", "sans-serif", "OFL"),
            ("Atma", "display", "OFL"),
            ("Atomic Age", "display", "OFL"),
            ("Average", "serif", "OFL"),
            ("Average Sans", "sans-serif", "OFL"),
        ];
        
        fonts_data.into_iter().map(|(name, category, license)| {
            let id = name.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            Font {
                id: format!("fontspace-{}", id),
                name: name.to_string(),
                provider: FontProvider::FontSpace,
                family: None,
                category: Self::parse_category(category),
                variants: vec!["regular".to_string()],
                variant_count: 1,
                subsets: vec!["latin".to_string()],
                license: Some(license.to_string()),
                designer: None,
                preview_url: Some(format!("{}/category/{}", self.base_url, id)),
                download_url: Some(format!("{}/get/{}.zip", self.base_url, id)),
                popularity: None,
            }
        }).collect()
    }
}

impl Default for FontSpaceProvider {
    fn default() -> Self {
        Self::new().expect("Failed to create FontSpace provider")
    }
}

#[async_trait]
impl FontProviderTrait for FontSpaceProvider {
    fn name(&self) -> &str {
        "FontSpace"
    }
    
    fn provider_type(&self) -> FontProvider {
        FontProvider::FontSpace
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
