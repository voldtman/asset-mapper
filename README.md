# asset-mapper

**Fast** and **efficient** in-memory & in-binary static files mapper macro for Rust, effectively eliminating file I/O at runtime.

## Overview

`asset-mapper` is a powerful Rust macro that scans your assets directory at compile time and generates a perfect hash function (PHF) map to embed your static files directly into your binary. This provides ultra-fast, zero-cost asset lookups and removes any runtime file system dependencies. Perfect for single binary server setups.

## Features

- **In-Memory Asset Storage:** Your static files are embedded in the binary, ensuring they are always available.
- **Zero Runtime I/O Overhead:** No file system access at runtime, giving you blazing fast response times.
- **Compile Time Optimization:** Uses PHF to generate a static map for ultra-fast lookups.
- **Automatic MIME Type Detection:** Automatically detects the correct MIME type for the assets.
- **Brotli Compression Support:** Recognizes `.br` files, allowing you to serve pre-compressed assets with proper HTTP headers.

## Example Usage

In your main project, you can use the macro to generate an embedded map of your assets, and write an in-memory static file handler that serves these embedded files via your HTTP server.

```rust
use asset_mapper::generate_asset_map;

// The macro generates a PHF map where each asset path maps to a tuple
// (asset bytes, content type, is_brotli_compressed).
static ASSETS: phf::Map<&'static str, (&'static [u8], &'static str, bool)> =
    generate_asset_map!("./dist");

async fn asset_handler(path: &str) -> Response {
    if let Some(&(content, content_type, is_br)) = ASSETS.get(path) {
        if is_br {
            return Response::new(Status::OK)
                .with_headers(|h| {
                    h.x("cache-control", "public, max-age=31536000, immutable")
                        .x("content-encoding", "br")
                })
                .with_payload(content_type, content);
        }
        Response::new(Status::OK)
            .with_headers(|h| h.x("cache-control", "public, max-age=31536000, immutable"))
            .with_payload(content_type, content)
    } else {
        Response::new(Status::NotFound)
    }
}
```
