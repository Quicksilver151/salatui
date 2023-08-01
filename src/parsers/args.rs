use clap::Parser;

/// A tui application to show and manage islamic prayer times
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to config file
    #[arg(short, long)]
    pub config: Option<String>,
    
}


