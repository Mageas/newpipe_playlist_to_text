use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path of the database file
    #[arg(short, long)]
    pub database: String,
    /// Path of the output directory
    #[arg(short, long)]
    pub output: Option<String>,
    /// Overwirite the existing playlists
    #[arg(long, default_value = "false")]
    pub overwrite: bool,
}
