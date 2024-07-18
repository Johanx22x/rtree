use std::fs;
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;

/// A customizable tree command built in Rust.
#[derive(Parser, Debug)]
#[clap(name = "rtree")]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    /// Enable hidden files
    all: bool,

    /// Enable colorized output
    #[arg(long, default_value_t = false)]
    colors: bool,

    #[arg(short, long, default_value_t = false)]
    /// Enable icons in output
    icons: bool,

    #[arg(short, long, default_value_t = false)]
    /// Enable less output
    less: bool,

    #[arg(short, long, default_value_t = false)]
    /// Show file sizes
    size: bool,

    #[arg(long, default_value_t = false)]
    /// Only display directories 
    dir_only: bool,

    #[arg(short, long, default_value_t = false)]
    /// Enable compress output
    compress: bool,

    #[arg(long, default_value_t = 5)]
    /// Amount of files to display per directory (compress mode)
    amount: u8,

    #[arg(long, default_value_t = false)]
    /// Enable compress directories (compress mode)
    dir_compress: bool,

    #[arg(long, default_value_t = 5)]
    /// Amount of directories to display per directory (compress dir mode)
    dir_amount: u8,

    #[arg(short, long, default_value_t = false)]
    /// Enable depth limit
    depth_limit: bool,

    #[arg(long, default_value_t = 5)]
    /// Depth limit 
    depth: u8,

    #[arg(short, long, default_value_t = false)]
    /// Enable name compression
    name_compress: bool,

    #[arg(long, default_value_t = 10)]
    /// Name compression limit 
    name_limit: u8,

    // TODO: Add pattern matching
}

fn main() {
    let args = Args::parse();
}
