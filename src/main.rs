use std::time::Duration;

// crates
pub use crossterm::{event, execute, terminal};

pub use event::{KeyCode, KeyModifiers, EnableMouseCapture, DisableMouseCapture, Event};
pub use terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
// pub use arboard::*;
pub use clap::Parser;
pub use tui::{
    Terminal,
    Frame,
    backend::{CrosstermBackend, Backend},
    layout::Alignment,
    style::{Color, Style},
    symbols::*,
    text::Line,
    widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Tabs},
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
    let timeset_data: TimeSetData = match &config.provider {
        Provider::Data(name) => TimeSetData::load(name).unwrap(),
        _ => todo!(),
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
    let mut app_state: AppState = AppState{config, timeset_data, ..Default::default()};
    
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
        // InputMap
        app_state.input_map.reset();
        app_state.input_char = char::default();
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                use input::*;
                match key.code {
                    KeyCode::Right => app_state.input_map.set_input(Key::Right), //app_state.day_offset += 1},
                    KeyCode::Left  => app_state.input_map.set_input(Key::Left), //app_state.day_offset -= 1},
                    
                    KeyCode::Enter => app_state.input_map.set_input(Key::Enter),
                    KeyCode::Backspace | KeyCode::Esc => app_state.input_map.set_input(Key::Escape),//app_state.day_offset = 0,
                    
                    KeyCode::Up   | KeyCode::BackTab => app_state.input_map.set_input(Key::Up),
                    KeyCode::Down | KeyCode::Tab     => app_state.input_map.set_input(Key::Down),
                    
                    KeyCode::Char(x) => app_state.input_map.set_input(Key::Command(x)),
                    _ => {}
                }
                match key.modifiers {
                    KeyModifiers::SHIFT => app_state.input_map.set_modifier(Modifier::Shift),
                    KeyModifiers::CONTROL => app_state.input_map.set_modifier(Modifier::Ctrl),
                    KeyModifiers::ALT => app_state.input_map.set_modifier(Modifier::Alt),
                    _ => {},
                }
            }
        }
        // dbg!(&input_map);
        
        // =====
        // Logic
        // =====
        let command = app_state.input_map.get_command();
        if let Some(command) = command {
            match command {
                'f' => app_state.fullscreen = !app_state.fullscreen,
                'q' => return Ok(()),
                 _  => {},
            };
        };
        
        if let Some(key) = app_state.input_map.get_key(){
        use input::{Key, Modifier};
        match key {
            (Key::Right, Modifier::Shift) => app_state.day_offset += 30,
            (Key::Left, Modifier::Shift)  => app_state.day_offset -= 30,
            (Key::Right, _) => app_state.day_offset += 1,
            (Key::Left, _)  => app_state.day_offset -= 1,
            (Key::Escape, _)=> app_state.day_offset =  0,
            _ => {},
        };
        }
        
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








