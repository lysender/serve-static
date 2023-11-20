use std::path::PathBuf;
use clap::Parser;

#[derive(Clone, Debug)]
pub struct Config {
    pub cors: bool,
    pub public: bool,
    pub dir: PathBuf,
    pub port: u16,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, &'static str> {
        // Check whether the dir is valid
        if !args.dir.is_dir() {
            return Err("Dir must exist.");
        }

        Ok(Config {
            cors: args.cors,
            public: args.public,
            dir: args.dir,
            port: args.port,
        })
    }
}

/// HTTP server that serves static files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short)]
    pub cors: bool,

    /// Expose to local network
    #[arg(long)]
    pub public: bool,

    #[arg(long, short)]
    pub dir: PathBuf,

    /// Local port
    #[arg(long, short, default_value_t = 3000)]
    pub port: u16,
}
