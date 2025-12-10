# Chrome Lens OCR

Library to use Google Lens OCR for free, via API used in Chromium. This doesn't require running a headless browser, and is much faster than using Puppeteer or similar.
It's set up to work without any options, there's no need to be authorized (no need for Google account!).

Port of <https://github.com/bropines/chrome-lens-py> to Rust

# Example

```bash
cargo run test.png
```

# Projects used in

- https://github.com/KolbyML/Mangatan
