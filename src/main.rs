use std::io::{Read, Result};
use std::fs::File;

use ratatui::{
    DefaultTerminal, Frame,
    text::Text,
    widgets::Block
};
use crossterm::event;

mod parser;
mod sim;
mod display;

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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut file = File::open("programs/default.wrapcode")?;
    let mut filestring = String::new();
    file.read_to_string(&mut filestring)?;
    let core: Vec<sim::Operation> = parser::parse(filestring).unwrap();
    println!("{}", core[0]);
    // ratatui::run(app)?;
    return Ok(());
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if event::read()?.is_key_press() {
            return Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let text = Text::raw(LOGO.to_string());
    let block = Block::bordered();
    let outer_area = frame.area();
    let inner_area = block.inner(outer_area);
    frame.render_widget(block, outer_area);
    frame.render_widget(text, inner_area);
}
