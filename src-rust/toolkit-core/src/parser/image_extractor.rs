use reqwest::blocking::get;
use kuchiki::traits::*;
use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::Path;

/* 
    Download images from an HTML string based on specified file extensions.

    # Arguments
    * `html` - A string slice that holds the HTML content to be parsed.
    * `extensions` - A string slice that contains comma-separated file extensions (e.g., "jpg,png").
    * `path` - A string slice that represents the directory where the images will be saved.
*/
pub fn download_images(html: &str, extensions: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extensions: Vec<&str> = extensions.split(',').collect();
    let document = kuchiki::parse_html().one(html);

    if !Path::new(path).exists() {
        create_dir_all(path)?;
    }

    for img_node in document.select("img").unwrap() {
        let img_element = img_node.as_node().as_element().unwrap();
        if let Some(src) = img_element.attributes.borrow().get("src") {
            if let Some(extension) = Path::new(src).extension() {
                if let Some(extension_str) = extension.to_str() {
                    if extensions.contains(&extension_str) {
                        let response = get(src)?;
                        let filename = Path::new(src).file_name().unwrap();
                        let target_path = Path::new(path).join(filename);
                        let mut file = File::create(target_path)?;
                        copy(&mut response.bytes()?.as_ref(), &mut file)?;
                    }
                }
            }
        }
    }

    Ok(())
}