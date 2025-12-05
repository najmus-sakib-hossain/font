//! dx-font - A comprehensive font search and download tool
//! 
//! Access 50k+ commercial-free fonts from 100+ sources including:
//! - Google Fonts (1,562 fonts)
//! - Bunny Fonts (1,478 fonts)
//! - Fontsource (1,562 fonts)
//! - Font Squirrel (1,082 fonts)
//! - And many more!

pub mod providers;
pub mod models;
pub mod search;
pub mod download;
pub mod cli;
pub mod config;

pub use models::{Font, FontFamily, FontProvider, FontWeight, FontStyle};
pub use search::FontSearch;
pub use download::FontDownloader;
