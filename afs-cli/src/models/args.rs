use clap::{self, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Arguments {
    /// 16 digit run code for the process
    pub run_code: String,
}
