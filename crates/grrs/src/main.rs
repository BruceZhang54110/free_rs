use clap::Parser;
use anyhow::{Context, Ok, Result};
use log::{info, warn};
use std::fmt::Write;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let line_count = content.lines().count();
    // 进度条
    let bar: ProgressBar = ProgressBar::new(line_count as u64);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut result = String::new();

    for line in content.lines() {
        // 进度条递增
        bar.inc(1);
        if line.contains(&args.pattern) {
            //println!("{}", line);
            result.push_str(line);
            result.push_str("\n");
        }
    }
    // 进度结束
    bar.finish();

    println!("search result: {}", result);
    Ok(())
}
