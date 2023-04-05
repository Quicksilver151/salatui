
// crates
pub use crossterm::{
    event::{self, KeyCode, EnableMouseCapture, DisableMouseCapture,
        Event::Key
    },
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub use tui::{
    Terminal,
    Frame,
    widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph},
    layout:: {Layout, Direction, Constraint, Rect},
    backend::{CrosstermBackend, Backend},
};


// mod files
mod ui;
mod salat;
mod parsers;

pub use ui::*;
pub use salat::*;
pub use parsers::*;






fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    let result = run_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    if let Err(e) = result {
        println!("{}", e.to_string());
    }
    
    
    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f))?;
        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc       =>  return Ok(()),
                KeyCode::Char('q') =>  return Ok(()),
                _ => {}
            }
        }
    }
}
fn new_block(title: &str) -> Block{
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    block
}

fn ui<B: Backend>(f: &mut Frame<B>){
    let layouts = MainLayout::from(f);
    let menu_block = new_block("Menu");
    
    let block_1 = new_block("1");
    let block_2 = new_block("2");
    let block_3 = new_block("3");
    let block_4 = new_block("4");
        
    
    // f.render_widget(menu_block, layouts.menu[0]);
    
    f.render_widget(block_1, layouts.settings[0]);
    f.render_widget(block_2, layouts.settings[1]);
    f.render_widget(block_3, layouts.settings[2]);
    f.render_widget(block_4, layouts.settings[3]);
    
}

// fn ui<B: Backend>(f: &mut Frame<B>) {
//     let parent_chunk: Vec<tui::layout::Rect> = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(
//             [
//                 Constraint::Percentage(50),
//                 Constraint::Percentage(50),
//             ].as_ref()
//         )
//         .split(f.size());
//     
//     let new_section_block = Block::default()
//         .title("Display")
//         .borders(Borders::ALL)
//         .border_type(BorderType::Rounded);
//     
//     f.render_widget(new_section_block, parent_chunk[0]);
//     
//     let list_section_block = Block::default()
//         .title(format!("Settings"))
//         .borders(Borders::ALL)
//         .border_type(BorderType::Rounded);
//     
//     f.render_widget(list_section_block, parent_chunk[1]);
//     
//         
//     
// }








