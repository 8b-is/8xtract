# Contributing to 8xtract

Thank you for your interest in contributing to 8xtract! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70+ (edition 2021)
- Git
- A running DeepSeek-OCR server for testing

### Clone and Build

```bash
git clone https://github.com/8b-is/8xtract.git
cd 8xtract
cargo build
```

### Run Tests

```bash
cargo test
```

### Build Release

```bash
cargo build --release
```

## Project Structure

```
8xtract/
├── Cargo.toml           # Workspace configuration
├── crates/
│   ├── core/            # Core OCR and extraction logic
│   │   ├── src/
│   │   │   ├── lib.rs          # Public API
│   │   │   ├── ocr.rs          # OCR client
│   │   │   └── extraction.rs   # Document extractor
│   │   └── Cargo.toml
│   ├── config/          # Configuration management
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── cli/             # Command-line interface
│       ├── src/main.rs
│       └── Cargo.toml
├── examples/            # Usage examples
└── README.md
```

## Code Style

- Follow Rust standard style guidelines
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Add documentation comments for public APIs
- Write tests for new functionality

### Formatting

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy -- -D warnings
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clear, concise code
- Add tests for new functionality
- Update documentation as needed
- Follow existing code patterns

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p xtract-core

# Build and test release version
cargo build --release
./target/release/8xtract --help
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "Clear description of your changes"
```

Use clear, descriptive commit messages:
- Start with a verb (Add, Fix, Update, Remove, etc.)
- Keep first line under 50 characters
- Add detailed description if needed

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Testing Guidelines

### Unit Tests

Add unit tests in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code here
    }
}
```

### Integration Tests

For larger integration tests, create a `tests/` directory in the crate.

## Documentation

- Add doc comments for public APIs using `///`
- Include examples in doc comments where helpful
- Update README.md for user-facing changes
- Update examples/README.md for usage changes

Example:

```rust
/// Extract text from an image file
/// 
/// # Arguments
/// 
/// * `image_path` - Path to the image file
/// * `prompt` - Optional custom OCR prompt
/// 
/// # Examples
/// 
/// ```no_run
/// use xtract_core::OcrClient;
/// use xtract_config::Config;
/// use std::path::Path;
/// 
/// let config = Config::default();
/// let client = OcrClient::new(config);
/// let text = client.extract_from_image(Path::new("doc.png"), None)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn extract_from_image(&self, image_path: &Path, prompt: Option<&str>) -> Result<String> {
    // Implementation
}
```

## Adding New Features

When adding new features:

1. **Discuss First**: For major changes, open an issue to discuss the approach
2. **Keep It Small**: Make focused, atomic changes
3. **Test Thoroughly**: Add comprehensive tests
4. **Document**: Update all relevant documentation
5. **Maintain Compatibility**: Avoid breaking changes when possible

## Code Review Process

- All changes require review before merging
- Address reviewer feedback promptly
- Keep discussions respectful and constructive
- Be open to suggestions and alternative approaches

## Release Process

Releases are managed by maintainers:

1. Version bump in Cargo.toml files
2. Update CHANGELOG.md
3. Create git tag
4. Build and test release artifacts
5. Publish to crates.io (if applicable)

## Getting Help

- Open an issue for bugs or feature requests
- Ask questions in pull request discussions
- Check existing issues and pull requests first

## License

By contributing to 8xtract, you agree that your contributions will be licensed under the Apache-2.0 license.

## Recognition

Contributors will be recognized in:
- Git commit history
- Release notes for significant contributions
- Project README (for major contributions)

Thank you for contributing to 8xtract!
