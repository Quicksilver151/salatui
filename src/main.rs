
// crates
use crossterm::{
    event::{self,
        KeyCode,
        EnableMouseCapture,
        DisableMouseCapture,
        Event::Key
    },
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen},
    
};
use tui::{
    Terminal,
    Frame,
    widgets::{
        Block,
        Borders,
        BorderType,
        List,
        ListItem,
        ListState,
        Paragraph,
    },
    backend::{
        CrosstermBackend,
        Backend
    }
    
};



fn main() {
    println!("Hello, world!");
}
