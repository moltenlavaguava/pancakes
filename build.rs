use ico::{IconDir, IconImage};
use image::GenericImageView;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    // this file was made with ai lol
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let icon_path = out_dir.join("icon.ico");

    let img = image::open("icon.png").expect("Failed to open PNG");
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);

    for size in &[16, 32, 48, 256] {
        let scaled = img.resize(*size, *size, image::imageops::FilterType::Lanczos3);
        let (width, height) = scaled.dimensions();
        let rgba = scaled.to_rgba8().into_raw();

        let icon_image = IconImage::from_rgba_data(width, height, rgba);
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).unwrap());
    }

    let file = File::create(&icon_path).expect("Failed to create .ico file");
    icon_dir.write(file).expect("Failed to write ICO data");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path.to_str().unwrap());
        res.compile().unwrap();
    }
}
