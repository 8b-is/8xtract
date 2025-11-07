# Examples

This directory contains example usage of 8xtract.

## Basic Usage

### 1. Start DeepSeek-OCR Server

First, ensure you have a DeepSeek-OCR server running. You can use the official Rust implementation:

```bash
# Clone and build deepseek-ocr.rs
git clone https://github.com/TimmyOVO/deepseek-ocr.rs.git
cd deepseek-ocr.rs

# Start the server (this will download models on first run)
cargo run -p deepseek-ocr-server --release -- --host 0.0.0.0 --port 8000
```

### 2. Extract Text from Documents

Once the server is running, use 8xtract to extract text:

```bash
# Extract from a single image
8xtract extract document.png

# Extract from multiple images
8xtract extract page1.png page2.png page3.png

# Save to file
8xtract extract invoice.jpg -o output.md

# Use custom prompt
8xtract extract receipt.png -p "Extract all items and prices from this receipt"
```

## Example Scenarios

### Receipt Processing

```bash
8xtract extract receipt.jpg -p "<|grounding|>Extract all items, quantities, and prices from this receipt and format as a markdown table"
```

### Invoice Extraction

```bash
8xtract extract invoice.pdf.png -p "<|grounding|>Extract invoice details including invoice number, date, items, and total amount"
```

### Form Data Extraction

```bash
8xtract extract form.png -p "<|grounding|>Extract all field names and values from this form"
```

### Document Digitization

```bash
# Batch process multiple document pages
8xtract extract page*.png -o digitized_document.md
```

## Configuration

View your current configuration:

```bash
8xtract config
```

Edit the configuration file directly:

```bash
# Get config path
CONFIG_PATH=$(8xtract config --path)

# Edit with your preferred editor
nano $CONFIG_PATH
# or
vim $CONFIG_PATH
```

### Custom API Endpoint

If your DeepSeek-OCR server is running on a different host/port:

Edit the config file and change:

```toml
[ocr]
api_endpoint = "http://your-server:8000/v1"
```

## Tips

1. **Enable verbose logging** for debugging:
   ```bash
   8xtract -v extract document.png
   ```

2. **Choose output format**:
   ```bash
   # Markdown (default)
   8xtract extract doc.png -f markdown
   
   # Plain text
   8xtract extract doc.png -f text
   ```

3. **Batch processing** - Use shell wildcards:
   ```bash
   8xtract extract *.png -o all_documents.md
   ```

4. **Custom prompts** - For specialized extraction, provide detailed instructions:
   ```bash
   8xtract extract table.png -p "Extract this table and convert to CSV format"
   ```

## Expected Output

The tool will output extracted text to stdout (default) or to a file if specified with `-o`.

For multiple images, each document is separated with headers showing the source filename:

```markdown
=== document1.png ===

[extracted text from document1]

---

=== document2.png ===

[extracted text from document2]
```

## Performance Notes

- First request to the DeepSeek-OCR server may be slow as models load
- Subsequent requests are much faster
- Batch processing processes images sequentially
- Large images may take longer to process

## Troubleshooting

If you encounter errors:

1. **Connection refused**: Ensure DeepSeek-OCR server is running
   ```bash
   curl http://localhost:8000/v1/models
   ```

2. **Image load errors**: Check that image files exist and are valid PNG/JPEG
   ```bash
   file your-image.png
   ```

3. **API errors**: Enable verbose logging to see detailed error messages
   ```bash
   8xtract -v extract document.png
   ```
