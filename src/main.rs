use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Clone)]
struct TodoItem {
    is_done: bool,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut state = AppState::default();

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
            if state.is_add_new {
                match add_new_hander(key, state) {
                    FormAction::Submit => {
                        state.is_add_new = false;

                        let mut last_item_number = state.items.len() + 1;

                        if state.items.len() > 0 {
                            last_item_number = indicator_reminder(state);
                        }

                        let task = TodoItem {
                            is_done: false,
                            description: format!(
                                "{}. {}",
                                last_item_number,
                                state.input_value.clone()
                            ),
                        };

                        state.items.push(task);
                        state.input_value.clear();
                        state.list_state.select(Some(last_item_number));
                    }
                    FormAction::Escape => {
                        state.is_add_new = false;
                        state.input_value.clear();
                    }
                    FormAction::None => {}
                }
            } else {
                if key_handler(key, state) {
                    break;
                };
            }
        }
    }

    Ok(())
}

fn indicator_reminder(state: &mut AppState) -> usize {
    if let Some(item) = state.items.last() {
        let denote: Vec<&str> = item.description.split(".").collect();

        if let Some(item_number) = denote.get(0).cloned() {
            let number: usize = match item_number.parse() {
                Ok(value) => value,
                Err(_) => 0,
            };

            return number + 1;
        }
    }

    return state.items.len();
}

fn add_new_hander(key: KeyEvent, state: &mut AppState) -> FormAction {
    match key.code {
        KeyCode::Enter => return FormAction::Submit,
        KeyCode::Esc => return FormAction::Escape,
        KeyCode::Backspace | KeyCode::Delete => {
            if !state.input_value.is_empty() {
                state.input_value.pop();
            }
        }
        KeyCode::Char(char) => {
            state.input_value.push(char);
        }
        _ => {}
    }

    FormAction::None
}

fn key_handler(key: KeyEvent, state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc => return true,
        KeyCode::Up => state.list_state.select_previous(),
        KeyCode::Down => state.list_state.select_next(),
        KeyCode::Char(char) => match char {
            'A' => state.is_add_new = true,
            'D' => {
                if let Some(index) = state.list_state.selected() {
                    state.items.remove(index);
                }
            }
            'q' => return true,
            _ => {}
        },
        KeyCode::Enter => {
            if let Some(index) = state.list_state.selected() {
                if let Some(item) = state.items.get_mut(index) {
                    item.is_done = !item.is_done;
                }
            }
        }
        _ => {}
    }

    false
}

fn render(frame: &mut Frame, state: &mut AppState) {
    if state.is_add_new {
        render_input_form(frame, state);
    } else {
        render_task_list(frame, state)
    }
}

fn render_input_form(frame: &mut Frame, state: &mut AppState) {
    Paragraph::new(state.input_value.as_str())
        .style(Style::default().fg(Color::Red))
        .block(
            Block::bordered()
                .title(
                    " Input Description "
                        .to_span()
                        .into_centered_line()
                        .bg(Color::LightCyan)
                        .fg(Color::Black),
                )
                .fg(Color::LightCyan)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        )
        .render(frame.area(), frame.buffer_mut());
}

fn render_task_list(frame: &mut Frame, state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .horizontal_margin(3)
        .vertical_margin(1)
        .areas(frame.area());

    let items = state.items.iter().map(|item| {
        let value = if item.is_done {
            item.description.to_span().crossed_out()
        } else {
            item.description.to_span()
        };

        ListItem::new(value)
    });

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" Todo List ")
                .padding(Padding::symmetric(3, 1))
                .border_type(BorderType::Rounded)
                .fg(Color::Yellow),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Green));

    frame.render_stateful_widget(list, border_area, &mut state.list_state);
}
