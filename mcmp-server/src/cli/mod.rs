

use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(long, short, default_value="./config.toml")]
    pub config: Utf8PathBuf,

    #[arg(long, short)]
    pub port: Option<u16>
}