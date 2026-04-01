use std::{
    fs::File,
    io::{Read, stdout}
};

use ratatui::{
    DefaultTerminal, Frame,
    text::Text,
    widgets::Block
};
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use color_eyre::eyre::Result;

use crate::sim::Operation;
use crate::parser::parse;

const LOGO: &str =
" _  _  ____   __   ____        _  _  ____   __
/ )( \\(  _ \\ / _\\ (  _ \\      / )( \\(  _ \\ / _\\
\\ /\\ / )   //    \\ ) __/ ____ \\ /\\ / )   //    \\
(_/\\_)(__\\_)\\_/\\_/(__)  (____)(_/\\_)(__\\_)\\_/\\_/
       _  _  ____   __   ____        _  _  ____
      / )( \\(  _ \\ / _\\ (  _ \\      / )( \\(  _ \\
 ____ \\ /\\ / )   //    \\ ) __/ ____ \\ /\\ / )   /
(____)(_/\\_)(__\\_)\\_/\\_/(__)  (____)(_/\\_)(__\\_)
 ____        _  _  ____   __   ____        _  _
(  _ \\      / )( \\(  _ \\ / _\\ (  _ \\      / )( \\
 ) __/ ____ \\ /\\ / )   //    \\ ) __/ ____ \\ /\\ /
(__)  (____)(_/\\_)(__\\_)\\_/\\_/(__)  (____)(_/\\_)
  __   ____        _  _  ____   __   ____
 / _\\ (  _ \\      / )( \\(  _ \\ / _\\ (  _ \\
/    \\ ) __/ ____ \\ /\\ / )   //    \\ ) __/ ____
\\_/\\_/(__)  (____)(_/\\_)(__\\_)\\_/\\_/(__)  (____)
 ____   __   ____        _  _  ____   __   ____
(  _ \\ / _\\ (  _ \\      / )( \\(  _ \\ / _\\ (  _ \\
 )   //    \\ ) __/ ____ \\ /\\ / )   //    \\ ) __/
(__\\_)\\_/\\_/(__)  (____)(_/\\_)(__\\_)\\_/\\_/(__)";


pub struct App {
    pub filepath: String,
    pub core: Vec<Operation>
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        use crossterm::event;
        let mut file = File::open(self.filepath.as_str())?;
        let mut filestring = String::new();
        file.read_to_string(&mut filestring)?;
        self.core = parse(filestring)?;
        stdout().execute(EnterAlternateScreen)?;
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if event::read()?.is_key_press() {
                stdout().execute(LeaveAlternateScreen)?;
                return Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let text = Text::raw(LOGO.to_string());
        let block = Block::bordered();
        let outer_area = frame.area();
        let inner_area = block.inner(outer_area);

        frame.render_widget(block, outer_area);
        frame.render_widget(text, inner_area);
    }
}
