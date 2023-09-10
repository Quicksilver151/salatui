use super::*;


#[derive(Default, Debug)]
pub struct Header {
    pub rect: Rect,
    pub text: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Menu,
    Settings,
    Calender,
}

#[derive(Default, Debug)]
pub struct Main {
    pub rect: Rect,
    pub screen: Screen,
}

#[derive(Default, Debug)]
pub struct Footer {
    pub commands: Vec<[String;2]>,
    pub rect: Rect,
}

#[derive(Default, Debug)]
pub struct UIState {
    pub header:Header,
    pub main: Main,
    pub footer: Footer,
}
impl UIState {
    pub fn set_header(&mut self, header_text: &str) {
        self.header.text = header_text.to_string();
    }
    pub fn set_screen(&mut self, screen: Screen) {
        self.main.screen = screen;
    }
    pub fn set_footer(&mut self, commands: Vec<[&str;2]>) {
        self.footer.commands = commands.iter().map(|v| [v[0].to_string(), v[1].to_string()]).collect();
    }
    
    
    pub fn set_header_rect(&mut self, rect: Rect) {
        self.header.rect = rect;
    }
    pub fn set_screen_rect(&mut self, rect: Rect) {
        self.main.rect = rect;
    }
    pub fn get_screen_rect(&mut self) -> Rect {
        self.main.rect
    }
    pub fn set_footer_rect(&mut self, rect: Rect) {
        self.footer.rect = rect;
    }
    
    pub fn get_footer_line(&self) -> Line {
        let mut spans = vec![];
        for letters in self.footer.commands.iter() {
            spans.append(&mut vec![
                Span::styled(letters[0].to_owned(), Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(letters[1].to_owned(), Style::default()),
            ]);
            if self.footer.commands.iter().last().unwrap() == letters {
                continue;
            }
            spans.append(&mut vec![
                Span::styled(" | ", Style::default()),
            ])
        }
        
        Line::from(spans)
    }

    pub fn get_header_line(&self) -> Line {
        Line::from(self.header.text.to_owned())
    }
    
    pub fn render_screen<B: Backend>(f: &mut Frame<B> ) {
        
    }
}
