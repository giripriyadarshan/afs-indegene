use clap::{self, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,

    /// 16 digit run code for the process
    run_code: String,

    #[clap(short, long, required = true)]
    /// svn url
    svn_path: String,
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
    /// array of key_messages
    key_messages: Vec<String>,
}
