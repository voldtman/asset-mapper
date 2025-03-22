# asset-mapper

**Fast** and **efficient** in-memory & in-binary static files mapper macro for Rust, effectively eliminating file I/O at runtime.

## Overview

`asset-mapper` is a powerful Rust macro that scans your assets directory at compile time and generates a perfect hash function (PHF) map to embed your static files directly into your binary. This provides ultra-fast, zero-cost asset lookups and removes any runtime file system dependencies.

## Features

- **In-Memory Asset Storage:** Your static files are embedded in the binary, ensuring they are always available.
- **Zero Runtime I/O Overhead:** No file system access at runtime, giving you blazing fast response times.
- **Compile Time Optimization:** Uses PHF to generate a static map for ultra-fast lookups.
- **Automatic MIME Type Detection:** Automatically detects the correct MIME type for the assets.
- **Brotli Compression Support:** Recognizes `.br` files, allowing you to serve pre-compressed assets with proper HTTP headers.
