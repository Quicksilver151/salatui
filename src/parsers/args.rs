use clap::Parser;

/// A tui application to show and manage islamic prayer times
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Output data once and exit program 
    #[arg(short, long, action)]
    pub output: bool,
    
    /// Use the config file from this path
    #[arg(short, long)]
    pub config: Option<std::path::PathBuf>,
    
}


