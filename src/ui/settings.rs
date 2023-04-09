
use crate::*;

pub fn get_settings_layout<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let mut container_size : Vec<f64> = vec![];
    
    container_size.append(&mut vec![100.0]);
    container_size.append(&mut vec![100.0]);
    container_size.append(&mut vec![100.0]);
    container_size.append(&mut vec![100.0]);
    
    let total = container_size[0]/100.0 + container_size[1]/100.0 + container_size[2]/100.0 + container_size[3]/100.0;
    
    let containers = container_size.iter().map(|x| (x/total) as u16 ).collect::<Vec<u16>>();
    
    
    // dbg!(total);
    // dbg!(&containers);

    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(containers[0]),
            Constraint::Percentage(containers[1]),
            Constraint::Percentage(containers[2]),
            Constraint::Percentage(containers[3]),
        ].as_ref()
    ).split(f.size())
}

