# Chrome Lens OCR

[![Crates.io](https://img.shields.io/crates/v/chrome_lens_ocr.svg)](https://crates.io/crates/chrome_lens_ocr)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Maintenance](https://img.shields.io/badge/maintenance-active-green.svg)

**A lightweight, high-performance Rust library to use Google Lens OCR.**

This crate allows you to extract text from images using the Google Lens API used in Chromium. It functions without a headless browser, making it significantly faster and lighter than Puppeteer-based solutions.

## ✨ Features

* **Zero Headless Browser:** No need for Selenium, Puppeteer, or Playwright.
* **No Authentication:** Works without a Google account or API keys.
* **High Performance:** Direct API calls make it fast and efficient.
* **Rust Port:** A direct port of the Python library [chrome-lens-py](https://github.com/bropines/chrome-lens-py).

---

## 📦 Installation

### As a Library
Add this to your `Cargo.toml`:

```toml
[dependencies]
chrome_lens_ocr = "0.1.0"

Or run the following command:

```bash
cargo add chrome_lens_ocr
```

### As a CLI Tool

To install the binary globally:

```bash
cargo install chrome_lens_ocr
```

-----

### Using the CLI

If you have installed the crate globally or are running it from the source:

```bash
# From source
cargo run -- test.png

# If installed globally
chrome_lens_ocr test.png
```

-----

## 🤝 Attribution

This project is a Rust port of [chrome-lens-py](https://github.com/bropines/chrome-lens-py) by [bropines](https://github.com/bropines).

## 🛠 Projects Used In

  * [Mangatan](https://github.com/KolbyML/Mangatan) - Easy to use all-in-one solution for reading Japanese manga with automatic OCR for seamless Yomitan lookups

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.

-----

**Disclaimer:** This project is intended for educational and experimental purposes only. Use of Google's services must comply with their Terms of Service. The author is not responsible for any misuse of this software.