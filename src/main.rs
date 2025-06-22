use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
