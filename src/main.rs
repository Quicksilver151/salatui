// crates
pub use crossterm::{event, execute, terminal};

pub use event::{KeyCode, EnableMouseCapture, DisableMouseCapture, Event::Key};
pub use terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
// pub use arboard::*;
pub use clap::Parser;
pub use tui::{
    Terminal,
    Frame,
    text::Spans,
    style::{Color, Style},
    symbols::*,
    widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Tabs},
    backend::{CrosstermBackend, Backend},
};

pub use serde::{Serialize, Deserialize};
pub use directories::*;
use chrono::*;

// mod files
mod structs;
mod ui;
mod salat;
mod parsers;

pub use structs::*;
pub use ui::*;
pub use salat::*;
pub use parsers::*;

fn output_data(config: &mut Config) {
    // clipboard testing
    // let mut x = arboard::Clipboard::new().unwrap();
    // let selection = LinuxClipboardKind::Primary;
    // arboard::Clipboard::get(&mut x);
    // println!("{:?}",x.get().clipboard(selection).text());
    // let load_dat = TimeSetData::load("GDh. Vilingili");
    // dbg!(load_dat);
    // conf.display.show_raw_output = true;
    // let timeset = MVRawData::default().parse_to_timeset(177).unwrap();
    // timeset.store(&timeset.name).unwrap();
    // conf.provider = Provider::Data(timeset.name.to_owned());
    
    // conf.provider = Provider::Calculation(CalculationMethod::default());
    // save_data(&conf);
    // let loaded: Config = confy::load("salatui", "config").unwrap();
    // salat_times(&conf, &timeset);
    let current_time = chrono::offset::Local::now();
    let current_day = current_time.ordinal0() as usize;
    // dbg!(current_time.num_seconds_from_midnight() / 60);
    // dbg!(current_date);
    
    match &config.provider {
        Provider::Data(name) => {
            let loaded = TimeSetData::load(name).unwrap();
            loop {
                let today_data = loaded.data_from_day(current_day);
                let today_data = today_data.output_format(config);
                println!("{}",today_data);
                if !config.raw_output.pool {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            // let today_dataset = PrayerTime::from_vec(loaded.data[current_date].clone());
        },
        
        Provider::Calculation(_) =>{},
    }
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::init().unwrap();
    let args = Args::parse();
    let mut config = match Config::load(){
        Ok(config) => config,
        Err(err) => {
            println!("{err}\nconfig is broken\nloading a new config from defaults");
            Config::default()
        }
    };
    
    if args.output {
        output_data(&mut config);
        // salat_times(&mut conf, timeset);
        return Ok(());
    }
    
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app_state: AppState = AppState::default();
    
    // main
    let result = run_app(&mut terminal, &mut app_state);
    
    // end
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    if let Err(e) = result {
        println!("{}", e);
    }
    
    Ok(())
}

// APPLICATION
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> Result<(), std::io::Error> {
    
    terminal.draw(|f| ui(f, app_state))?;
    loop {
        app_state.input_map.reset();
        app_state.input_char = char::default();
        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Right | KeyCode::Enter => app_state.input_map.enter = true,
                KeyCode::Left  | KeyCode::Esc  | KeyCode::Backspace => app_state.input_map.back  = true,
                
                KeyCode::Up   | KeyCode::BackTab => app_state.input_map.up   = true,
                KeyCode::Down | KeyCode::Tab     => app_state.input_map.down = true,
                
                KeyCode::Char(x) => app_state.input_char = x,
                _ => {}
            }
            
        }
        // dbg!(&input_map);
        if app_state.input_char == 'f' { app_state.fullscreen = !app_state.fullscreen };
        if app_state.input_char == 'q' { return Ok(()) };
        terminal.draw(|f| ui(f, app_state))?;
    }
}

// INTERFACE

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








