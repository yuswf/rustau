use reqwest;
use scraper;
use tokio;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut audios: Vec<String> = Vec::new();
    let categories: [&str; 5] = [
        "Sound_effects",
        "Item_sounds",
        "Block_sounds",
        "Historical_sounds",
        "Nether_wastes_ambient_sounds",
    ];

    for category in &categories {
        let url = format!("https://minecraft.fandom.com/wiki/Category:{}", category);
        let body = reqwest::get(&url).await?.text().await?;
        let doc = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("audio").unwrap();

        for element in doc.select(&selector) {
            let src = element.value().attr("src").unwrap();

            audios.push(src.to_string());
        }
    }

    for (i, audio) in audios.iter().enumerate() {
        let response = reqwest::get(audio).await?;
        let bytes = response.bytes().await?;
        // create the audios directory first in root
        let mut file = File::create(Path::new(&format!("audios/audio_{}.mp3", i + 1)))?;

        file.write_all(&bytes).unwrap();

        println!("Downloaded {}/{}", i + 1, audios.len());
    }

    Ok(())
}
