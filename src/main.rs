use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Widget},
};

#[derive(Debug)]
struct TodoItem {
    is_done: bool,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut state = AppState::default();

    // Select the first item
    state.list_state.select(Some(0));
    state.is_add_new = false;

    state.items.push(TodoItem {
        is_done: false,
        description: "Running Application".into(),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: "Running Application".into(),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: "Running Application".into(),
    });

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|frame| render(frame, state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char(char) => match char {
                    'j' => state.list_state.select_previous(),
                    'k' => state.list_state.select_next(),
                    'D' => {
                        if let Some(index) = state.list_state.selected() {
                            state.items.remove(index);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .horizontal_margin(1)
        .vertical_margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let items = state
        .items
        .iter()
        .map(|item| ListItem::new(item.description.clone()));

    let list = List::new(items).highlight_style(Style::default().fg(Color::Black).bg(Color::Green));

    frame.render_stateful_widget(list, inner_area, &mut state.list_state);
}
