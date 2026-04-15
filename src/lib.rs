use rust_embed::RustEmbed;

pub mod service;
pub mod util;

#[derive(RustEmbed)]
#[folder = "include/"]
struct IncludeFiles;
