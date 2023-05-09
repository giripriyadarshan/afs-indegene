use clap::{self, Parser};

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Arguments {
    #[clap(short, long, required = true)]
    /// should upload?
    pub upload: bool,

    #[clap(short, long, requires = "upload")]
    /// svn url
    pub url: String,

    #[clap(short, long, requires = "upload")]
    /// array of key_messages
    pub key_messages: Vec<String>,

    #[clap(short, long, requires = "upload")]
    /// if thumb is true, will generate thumb
    pub thumb: bool,

    #[clap(short, long, requires = "thumb")]
    /// if thumb is true, will accept thumb_width
    pub thumb_width: u16,

    #[clap(short, long, requires = "thumb")]
    /// if thumb is true, will accept thumb_height
    pub thumb_height: u16,

    #[clap(short, long, requires = "thumb")]
    /// if "pdf" will generate using pdf file, if "browser" will generate using browser backend
    pub thumb_method: String,

    #[clap(short, long, required = false)]
    /// get_key_messages
    pub get_key_messages: bool,
}