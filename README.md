# 8xtract

The Mem8 Document Extractor - A high-performance OCR-powered document extraction tool inspired by DeepSeek-OCR.

## Overview

8xtract is a Rust-based document extraction system that leverages advanced OCR technology to extract text and structure from document images. It provides a clean CLI interface and can be configured to work with DeepSeek-OCR or compatible OCR services.

## Features

- **Fast OCR Extraction**: Leverages DeepSeek-OCR's advanced vision-language model for accurate text extraction
- **Markdown Conversion**: Automatically converts documents to markdown while preserving layout and structure
- **Batch Processing**: Process multiple images in a single command
- **Flexible Configuration**: Easy-to-use configuration system with sensible defaults
- **Multiple Output Formats**: Support for plain text and markdown output
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Architecture

8xtract is built with a modular architecture inspired by deepseek-ocr.rs:

- **xtract-core**: Core OCR client and document extraction logic
- **xtract-config**: Configuration management
- **xtract-cli**: Command-line interface

## Prerequisites

- Rust 1.70+ (edition 2021)
- A running instance of DeepSeek-OCR server (or compatible OCR API)

## Installation

### From Source

```bash
git clone https://github.com/8b-is/8xtract.git
cd 8xtract
cargo build --release
```

The binary will be available at `target/release/8xtract`.

### Install Globally

```bash
cargo install --path crates/cli
```

## Quick Start

### 1. Set Up DeepSeek-OCR Server

First, you need a running DeepSeek-OCR server. You can use the deepseek-ocr.rs project:

```bash
# Clone and run deepseek-ocr.rs server
git clone https://github.com/TimmyOVO/deepseek-ocr.rs.git
cd deepseek-ocr.rs
cargo run -p deepseek-ocr-server --release -- --host 0.0.0.0 --port 8000
```

### 2. Extract Text from Documents

```bash
# Extract text from a single image
8xtract extract image.png

# Extract from multiple images
8xtract extract doc1.png doc2.png doc3.jpg

# Save output to a file
8xtract extract receipt.png -o output.md

# Use a custom prompt
8xtract extract invoice.png -p "Extract all text and convert to structured markdown"

# Specify output format
8xtract extract document.png -f markdown -o result.md
```

## Configuration

8xtract stores its configuration in:
- Linux: `~/.config/xtract/config.toml`
- macOS: `~/Library/Application Support/xtract/config.toml`
- Windows: `%APPDATA%\xtract\config.toml`

### View Configuration

```bash
# Show current configuration
8xtract config

# Show configuration file path
8xtract config --path
```

### Default Configuration

```toml
[ocr]
api_endpoint = "http://localhost:8000/v1"
model = "deepseek-ocr"
max_tokens = 512
temperature = 0.0

[extraction]
output_format = "markdown"
preserve_layout = true
```

### Configuration Options

#### OCR Settings

- `api_endpoint`: URL of the OCR API server
- `model`: Model name to use (e.g., "deepseek-ocr")
- `max_tokens`: Maximum number of tokens to generate
- `temperature`: Sampling temperature (0.0 for deterministic output)

#### Extraction Settings

- `output_format`: Output format ("text" or "markdown")
- `preserve_layout`: Whether to preserve document layout in output

## Usage Examples

### Basic Text Extraction

```bash
8xtract extract document.png
```

### Convert Receipt to Markdown

```bash
8xtract extract receipt.jpg -o receipt.md
```

### Batch Process Documents

```bash
8xtract extract page*.png -o combined.md
```

### Custom OCR Prompts

For specialized extraction tasks, you can provide custom prompts:

```bash
# Extract and structure as a table
8xtract extract table.png -p "<|grounding|>Extract this table and format as markdown"

# Extract with specific instructions
8xtract extract form.png -p "Extract all fields from this form and organize them"
```

## Integration with DeepSeek-OCR

8xtract is designed to work seamlessly with DeepSeek-OCR.rs, leveraging its powerful features:

- **Vision-Language Model**: Advanced OCR using transformer-based models
- **Layout Preservation**: Maintains document structure including tables and columns
- **Multi-Language Support**: Handles 90+ languages including CJK and RTL scripts
- **High Accuracy**: ~97% accuracy with context-aware extraction

### DeepSeek-OCR Features Utilized

- OpenAI-compatible API for easy integration
- Markdown conversion with table support
- Spatial layout understanding
- Context optical compression for efficiency

## Performance

8xtract is built with performance in mind:

- Written in Rust for maximum speed and safety
- Efficient HTTP client with connection pooling
- Minimal memory footprint
- Fast image loading and encoding

## Use Cases

- **Document Digitization**: Convert paper documents to searchable text
- **Receipt Processing**: Extract structured data from receipts and invoices
- **Form Extraction**: Pull information from filled forms
- **Archive Scanning**: Batch process historical documents
- **Data Entry Automation**: Eliminate manual transcription work

## API Compatibility

8xtract uses the OpenAI-compatible chat completion API format, making it compatible with:

- DeepSeek-OCR.rs
- Any OpenAI-compatible vision API
- Custom OCR services implementing the same interface

## Development

### Building

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Build specific crate
cargo build -p xtract-cli
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p xtract-core
```

### Project Structure

```
8xtract/
├── Cargo.toml           # Workspace configuration
├── crates/
│   ├── core/            # Core OCR and extraction logic
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── ocr.rs
│   │   │   └── extraction.rs
│   │   └── Cargo.toml
│   ├── config/          # Configuration management
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── cli/             # Command-line interface
│       ├── src/main.rs
│       └── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

Apache-2.0

## Acknowledgments

This project is inspired by and designed to work with:
- [DeepSeek-OCR.rs](https://github.com/TimmyOVO/deepseek-ocr.rs) - Rust implementation of DeepSeek-OCR
- [DeepSeek-OCR](https://github.com/deepseek-ai/DeepSeek-OCR) - The original DeepSeek-OCR project

## Related Projects

- [deepseek-ocr.rs](https://github.com/TimmyOVO/deepseek-ocr.rs) - Rust implementation with CLI and server
- [DeepSeek-OCR](https://github.com/deepseek-ai/DeepSeek-OCR) - Original Python implementation

