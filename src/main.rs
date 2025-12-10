use std::env;

use chrome_lens_ocr::LensClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_image>", args[0]);
        return Ok(());
    }

    let image_path = &args[1];

    // Initialize client (uses default API Key if None provided)
    let client = LensClient::new(None);

    println!("Processing image: {}", image_path);
    match client.process_image_path(image_path, Some("en")).await {
        Ok(text) => {
            println!("--- OCR Result ---");
            println!("{}", text);
            println!("------------------");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
