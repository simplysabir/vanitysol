use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    vanity: String,

    #[clap(subcommand)]
    command: CommandEnum,
}

#[derive(clap::Subcommand, Debug)]
enum CommandEnum {
    Generate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CommandEnum::Generate => generate_vanity_address(&cli.vanity)?,
    }

    Ok(())
}

fn generate_vanity_address(vanity: &str) -> Result<()> {
    let command = format!("solana-keygen grind --starts-with {}:1", vanity);
    
    // Execute the solana-keygen command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .with_context(|| "Failed to execute solana-keygen")?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("solana-keygen output:\n{}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("solana-keygen grind failed: {}", error_str));
    }

    Ok(())
}
