use chrome_lens_ocr::LensClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_image>", args[0]);
        return Ok(());
    }

    let image_path = &args[1];

    let client = LensClient::new(None);

    println!("Processing image: {}", image_path);
    match client.process_image_path(image_path, Some("en")).await {
        Ok(result) => {
            println!("--- Full Text ---");
            println!("{}", result.full_text);

            println!("\n--- Detailed Structure ---");
            println!("Found {} paragraphs.", result.paragraphs.len());
            for (i, para) in result.paragraphs.iter().enumerate() {
                println!("Paragraph {}: {} lines", i + 1, para.lines.len());
                if let Some(first_line) = para.lines.first() {
                    if let Some(geom) = &first_line.geometry {
                        println!(
                            "  -> First line pos: x={:.2}, y={:.2}, w={:.2}",
                            geom.center_x, geom.center_y, geom.width
                        );
                    }
                }
            }

            if let Some(trans) = result.translation {
                println!("\n--- Translation ---");
                println!("{}", trans);
            }
            println!("------------------");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
