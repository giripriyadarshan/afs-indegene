use clap::{self, Parser};

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Arguments {
    #[clap(short, long, required = true)]
    /// svn url
    pub url: String,

    #[clap(short, long, required = true)]
    /// array of key_messages
    pub key_messages: Vec<String>,

    #[clap(short, long, required = false)]
    /// if thumb is true, will generate thumb
    pub thumb: bool,

    #[clap(short, long, requires = "thumb")]
    /// if thumb is true, will accept thumb_width
    pub thumb_width: u16,

    #[clap(short, long, requires = "thumb")]
    /// if thumb is true, will accept thumb_height
    pub thumb_height: u16,
}