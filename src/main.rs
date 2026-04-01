mod display;
mod parser;
mod sim;
mod tui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let path = "programs/default.wrapcode".to_string();
    let mut app = tui::App { filepath: path, core: Vec::new() };
    let terminal = ratatui::init();
    app.run(terminal)?;
    ratatui::restore();
    return Ok(());
}
