use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)
        .with_context(|| format!("Couldn't read file: `{}`", &args.path.display()))?;
    let reader = BufReader::new(f);
    find_matches(reader, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}
