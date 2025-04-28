# Rust Sitemap Checker

A command-line tool that checks the validity of URLs in a website's sitemap.

## Description

This Rust application fetches a website's sitemap.xml file, parses it to extract URLs, and can verify their accessibility. It also respects the website's robots.txt file for crawl delay settings.

## Features

- Fetches and parses XML sitemaps
- Respects robots.txt crawl-delay directive
- Configurable via environment variables
- Detailed logging

## Prerequisites

- Rust and Cargo installed
- Internet connection to fetch sitemaps and check URLs

## Installation

Clone the repository:

```bash
git clone https://github.com/kaidoj/rust-sitemap-checker.git
cd rust-sitemap-checker
```

Build the project:

```bash
cargo build
```

## Configuration

Create a `.env` file in the project root with the following variables:

```
WEBSITE_URL=https://example.com
SITEMAP_LOCATION=https://example.com/sitemap.xml
CRAWL_DELAY=2
RUST_LOG=info
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| WEBSITE_URL | The base URL of the website | https://example.com |
| SITEMAP_LOCATION | The full URL to the sitemap.xml file | https://example.com/sitemap.xml |
| CRAWL_DELAY | Default delay between requests in seconds | 2 |
| RUST_LOG | Log level (trace, debug, info, warn, error) | info |

## Usage

Run the application:

```bash
cargo run
```

Or use the compiled binary directly:

```bash
./target/debug/rust-sitemap-checker
```

For a release build with optimizations:

```bash
cargo build --release
./target/release/rust-sitemap-checker
```

## License

[MIT License](LICENSE)