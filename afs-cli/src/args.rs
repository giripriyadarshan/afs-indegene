use clap::{self, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,

    /// 16 digit run code for the process
    run_code: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "upload", alias = "u")]
    /// can also use 'u'
    Upload(Upload),

    #[clap(name = "get_key_messages", alias = "gkm")]
    /// can also use 'gkm'
    GetKeyMessages,
}

#[derive(Parser, Debug)]
pub struct Upload {
    #[clap(short, long, required = true)]
    /// svn url
    svn_path: String,

    #[clap(short, long, required = true)]
    /// array of key_messages
    key_messages: Vec<String>,

    #[clap(short, long)]
    /// if thumb is true, will generate thumb
    thumb: bool,

    #[clap(short = 'w', long, default_value = "0")]
    /// if thumb is true, will accept thumb_width
    thumb_width: u16,

    #[clap(short = 'e', long, default_value = "0")]
    /// if thumb is true, will accept thumb_height
    thumb_height: u16,

    #[clap(short = 'm', long, default_value = "pdf")]
    /// if "pdf" will generate using pdf file, if "browser" will generate using browser backend
    thumb_method: String,
}
