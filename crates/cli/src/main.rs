use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use xtract_config::Config;
use xtract_core::DocumentExtractor;

#[derive(Parser)]
#[command(name = "8xtract")]
#[command(author, version, about = "The Mem8 Document Extractor - OCR-powered document extraction", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract text from document images
    Extract {
        /// Input image file(s)
        #[arg(required = true)]
        images: Vec<PathBuf>,

        /// Custom OCR prompt (optional)
        #[arg(short, long)]
        prompt: Option<String>,

        /// Output file (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format (text, markdown)
        #[arg(short, long)]
        format: Option<String>,
    },

    /// Show configuration
    Config {
        /// Show the configuration file path
        #[arg(long)]
        path: bool,
    },
}

fn setup_logging(verbose: bool) {
    let level = if verbose { Level::DEBUG } else { Level::INFO };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    match cli.command {
        Commands::Extract {
            images,
            prompt,
            output,
            format,
        } => {
            info!("Starting document extraction");
            
            // Load configuration
            let mut config = Config::load()
                .context("Failed to load configuration")?;

            // Override format if specified
            if let Some(fmt) = format {
                config.extraction.output_format = fmt;
            }

            // Create extractor
            let extractor = DocumentExtractor::new(config);

            // Extract from images
            let image_refs: Vec<&std::path::Path> = images.iter().map(|p| p.as_path()).collect();
            let documents = extractor.extract_batch(&image_refs, prompt.as_deref())
                .context("Failed to extract documents")?;

            // Output results
            if documents.is_empty() {
                eprintln!("Warning: No documents were successfully extracted");
                return Ok(());
            }

            let combined_text = documents
                .iter()
                .map(|d| {
                    format!(
                        "=== {} ===\n\n{}\n",
                        d.metadata.source,
                        d.text
                    )
                })
                .collect::<Vec<_>>()
                .join("\n---\n\n");

            if let Some(output_path) = output {
                std::fs::write(&output_path, &combined_text)
                    .with_context(|| format!("Failed to write to {:?}", output_path))?;
                info!("Results written to: {:?}", output_path);
            } else {
                println!("{}", combined_text);
            }

            info!("Extraction completed successfully");
        }

        Commands::Config { path } => {
            if path {
                let config_path = Config::config_path()?;
                println!("{}", config_path.display());
            } else {
                let config = Config::load()
                    .context("Failed to load configuration")?;
                let config_str = toml::to_string_pretty(&config)
                    .context("Failed to serialize configuration for display")?;
                println!("{}", config_str);
            }
        }
    }

    Ok(())
}
