use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Wrap},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    is_deleting: bool,
    is_open: bool,
    is_editing: bool,
    input_state: InputState,
}

#[derive(Debug, Default)]
struct InputState {
    name_input: String,
    description_input: String,
    select_state: InputSelectState,
}

#[derive(Debug, Default, PartialEq)]
enum InputSelectState {
    #[default]
    Name,
    Description,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    name: String,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;

    let terminal = ratatui::init();
    let res = run(terminal, &mut state);

    ratatui::restore();
    res
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        //Redering
        terminal.draw(|f| render(f, app_state))?;
        //Input handling
        if let Event::Key(k) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(k, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_add_new = false;

                        app_state.items.push(TodoItem {
                            is_done: false,
                            name: app_state.input_state.name_input.clone(),
                            description: app_state.input_state.description_input.clone(),
                        });

                        app_state.input_state.name_input.clear();
                        app_state.input_state.description_input.clear();
                        app_state.input_state.select_state = InputSelectState::Name;
                    }
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_state.name_input.clear();
                        app_state.input_state.description_input.clear();
                        app_state.input_state.select_state = InputSelectState::Name;
                    }
                }
            } else if app_state.is_editing {
                match handle_edit(k, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_editing = false;

                        if let Some(item) = app_state
                            .list_state
                            .selected()
                            .and_then(|idx| app_state.items.get_mut(idx))
                        {
                            item.name = app_state.input_state.name_input.clone();
                            item.description = app_state.input_state.description_input.clone();
                        }

                        app_state.input_state.name_input.clear();
                        app_state.input_state.description_input.clear();
                        app_state.input_state.select_state = InputSelectState::Name;
                    }
                    FormAction::Escape => {
                        app_state.is_editing = false;
                        app_state.input_state.name_input.clear();
                        app_state.input_state.description_input.clear();
                        app_state.input_state.select_state = InputSelectState::Name;
                    }
                }
            } else if app_state.is_deleting {
                handle_delete(k, app_state);
            } else if app_state.is_open {
                handle_open(k, app_state);
            } else {
                if handle_key(k, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_open(k: KeyEvent, app_state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => {
            app_state.is_open = false;
        }
        _ => {}
    }
    false
}

fn handle_add_new(k: KeyEvent, app_state: &mut AppState) -> FormAction {
    match k.code {
        event::KeyCode::Char(c) => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.name_input.push(c)
            } else {
                app_state.input_state.description_input.push(c)
            };
        }
        event::KeyCode::Backspace => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.name_input.pop()
            } else {
                app_state.input_state.description_input.pop()
            };
        }
        event::KeyCode::Enter => {
            if app_state.input_state.name_input.is_empty() {
                return FormAction::None;
            } else {
                return FormAction::Submit;
            }
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        event::KeyCode::Tab => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.select_state = InputSelectState::Description
            } else {
                app_state.input_state.select_state = InputSelectState::Name
            }
        }
        _ => {}
    }

    FormAction::None
}

fn handle_edit(k: KeyEvent, app_state: &mut AppState) -> FormAction {
    match k.code {
        event::KeyCode::Char(c) => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.name_input.push(c)
            } else {
                app_state.input_state.description_input.push(c)
            };
        }
        event::KeyCode::Backspace => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.name_input.pop()
            } else {
                app_state.input_state.description_input.pop()
            };
        }
        event::KeyCode::Enter => {
            if app_state.input_state.name_input.is_empty() {
                return FormAction::None;
            } else {
                return FormAction::Submit;
            }
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        event::KeyCode::Tab => {
            if app_state.input_state.select_state == InputSelectState::Name {
                app_state.input_state.select_state = InputSelectState::Description
            } else {
                app_state.input_state.select_state = InputSelectState::Name
            }
        }
        _ => {}
    }

    FormAction::None
}

fn handle_delete(k: KeyEvent, app_state: &mut AppState) -> bool {
    if let event::KeyCode::Char(c) = k.code {
        match c {
            'y' => {
                if let Some(idx) = app_state.list_state.selected() {
                    app_state.items.remove(idx);
                }
                app_state.is_deleting = false;
            }
            'n' => {
                app_state.is_deleting = false;
            }
            _ => {}
        }
    }
    false
}

fn handle_key(k: KeyEvent, app_state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Enter => {
            if app_state.list_state.selected().is_some() {
                app_state.is_open = true;
            }
        }
        event::KeyCode::Char(c) => match c {
            'a' => {
                app_state.is_add_new = true;
            }
            'd' => {
                if app_state.list_state.selected().is_some() {
                    app_state.is_deleting = true;
                }
            }
            'e' => {
                if let Some(item) = app_state
                    .list_state
                    .selected()
                    .and_then(|idx| app_state.items.get(idx))
                {
                    app_state.is_editing = true;
                    app_state.input_state.name_input = item.name.clone();
                    app_state.input_state.description_input = item.description.clone();
                }
            }
            'c' => {
                if let Some(item) = app_state
                    .list_state
                    .selected()
                    .and_then(|idx| app_state.items.get_mut(idx))
                {
                    item.is_done = !item.is_done;
                }
            }
            'j' => {
                app_state.list_state.select_next();
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            _ => {}
        },
        _ => {}
    }

    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(
            " TUIDoList "
                .to_span()
                .into_centered_line()
                .fg(Color::Yellow),
        )
        .title_bottom(
            (" Up ".to_span().fg(Color::Yellow)
                + "[k]".to_span().fg(Color::Green)
                + " Down ".to_span().fg(Color::Yellow)
                + "[j]".to_span().fg(Color::Green)
                + " New ".to_span().fg(Color::Yellow)
                + "[a]".to_span().fg(Color::Green)
                + " Edit ".to_span().fg(Color::Yellow)
                + "[e]".to_span().fg(Color::Green)
                + " Delete ".to_span().fg(Color::Yellow)
                + "[d]".to_span().fg(Color::Green)
                + " Complete ".to_span().fg(Color::Yellow)
                + "[c]".to_span().fg(Color::Green)
                + " Exit ".to_span().fg(Color::Yellow)
                + "[Esc] ".to_span().fg(Color::Green))
            .alignment(ratatui::layout::HorizontalAlignment::Center),
        )
        .fg(Color::Cyan)
        .render(border_area, frame.buffer_mut());
    render_list(frame, app_state);

    if app_state.is_add_new {
        render_add(frame, app_state);
    }

    if app_state.is_editing {
        render_edit(frame, app_state);
    }

    if app_state.is_deleting {
        render_delete(frame, app_state);
    }

    if app_state.is_open {
        render_item(frame, app_state);
    }
}

fn render_add(frame: &mut Frame, app_state: &mut AppState) {
    let area = frame.area();
    let popup_width = (area.width as f32 * 0.3) as u16;
    let popup_height = (area.height as f32 * 0.4) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = ratatui::layout::Rect::new(popup_x, popup_y, popup_width, popup_height);

    let popup_block = Block::bordered()
        .title(
            " Add New Item "
                .to_span()
                .fg(Color::Yellow)
                .into_centered_line(),
        )
        .title_bottom(
            (" Next ".to_span().fg(Color::Yellow)
                + "[Tab]".to_span().fg(Color::Green)
                + " Submit ".to_span().fg(Color::Yellow)
                + "[Enter]".to_span().fg(Color::Green)
                + " Cancel ".to_span().fg(Color::Yellow)
                + "[Esc] ".to_span().fg(Color::Green))
            .alignment(ratatui::layout::HorizontalAlignment::Center),
        )
        .border_type(BorderType::Rounded)
        .fg(Color::Cyan);

    frame.render_widget(popup_block, popup_area);

    let [title_area, description_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
            .margin(1)
            .areas(popup_area);

    Paragraph::new(
        " ".to_span()
            + app_state
                .input_state
                .name_input
                .as_str()
                .fg(Color::default())
            + if app_state.input_state.select_state == InputSelectState::Name {
                "|".to_span().fg(Color::default())
            } else {
                "".to_span()
            },
    )
    .scroll((
        0,
        app_state
            .input_state
            .name_input
            .len()
            .saturating_sub(title_area.width as usize - 4) as u16,
    ))
    .block(
        Block::bordered()
            .title(" Title ".fg(Color::Yellow))
            .fg(
                if app_state.input_state.select_state == InputSelectState::Name {
                    Color::White
                } else {
                    Color::Green
                },
            )
            .border_type(BorderType::Rounded),
    )
    .render(title_area, frame.buffer_mut());

    Paragraph::new(
        app_state
            .input_state
            .description_input
            .as_str()
            .fg(Color::default())
            + if app_state.input_state.select_state == InputSelectState::Description {
                "|".to_span().fg(Color::default())
            } else {
                "".to_span()
            },
    )
    .wrap(Wrap { trim: false })
    .block(
        Block::bordered()
            .title(" Description ".fg(Color::Yellow))
            .fg(
                if app_state.input_state.select_state == InputSelectState::Description {
                    Color::White
                } else {
                    Color::Green
                },
            )
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded),
    )
    .render(description_area, frame.buffer_mut());
}

fn render_list(frame: &mut Frame, app_state: &mut AppState) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(3)
        .areas(frame.area());

    let list = List::new(app_state.items.iter().map(|x| {
        let value = if x.is_done {
            x.name.to_span().crossed_out()
        } else {
            x.name.to_span()
        };

        ListItem::from(value).fg(Color::default())
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Green));

    if app_state.items.is_empty() && !app_state.is_add_new {
        let para = Paragraph::new("all done :)".to_span().fg(Color::default()))
            .alignment(ratatui::layout::HorizontalAlignment::Center);

        let vertical_offset = (inner_area.height.saturating_sub(1)) / 2;

        let centered_area = ratatui::layout::Rect::new(
            inner_area.x,
            inner_area.y + vertical_offset,
            inner_area.width,
            1,
        );

        frame.render_widget(para, centered_area);
    } else {
        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}

fn render_delete(frame: &mut Frame, app_state: &mut AppState) {
    let area = frame.area();
    let selected_item_name = app_state
        .list_state
        .selected()
        .and_then(|idx| app_state.items.get(idx))
        .map(|item| item.name.clone())
        .unwrap_or(String::from("Unnamed Item"));

    let text_line =
        "Delete Item: ".to_span().fg(Color::Yellow) + selected_item_name.to_span().fg(Color::Green);

    let text_width = text_line.width() as u16;

    let popup_width = text_width + 6;
    let popup_height = 5;

    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.height / 4;

    let popup_area = ratatui::layout::Rect::new(popup_x, popup_y, popup_width, popup_height);

    Paragraph::new(text_line.alignment(ratatui::layout::HorizontalAlignment::Center))
        .block(
            Block::bordered()
                .fg(Color::Cyan)
                .padding(Padding::uniform(1))
                .title(" Delete ".to_span().into_centered_line())
                .title_bottom(
                    (" Yes ".to_span().fg(Color::Yellow)
                        + "[y]".to_span().fg(Color::Green)
                        + " No ".to_span().fg(Color::Yellow)
                        + "[n] ".to_span().fg(Color::Green))
                    .alignment(ratatui::layout::HorizontalAlignment::Center),
                )
                .border_type(BorderType::Rounded),
        )
        .render(popup_area, frame.buffer_mut());
}

fn render_item(frame: &mut Frame, app_state: &mut AppState) {
    let item = app_state
        .list_state
        .selected()
        .and_then(|idx| app_state.items.get_mut(idx))
        .unwrap();

    let area = frame.area();
    let popup_width = (area.width as f32 * 0.3) as u16;
    let popup_height = (area.height as f32 * 0.4) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = ratatui::layout::Rect::new(popup_x, popup_y, popup_width, popup_height);

    let title = if item.is_done { " Done " } else { " To Do " };

    let popup_block = Block::bordered()
        .title(title.to_span().fg(Color::Yellow).into_centered_line())
        .title_bottom(
            (" Close ".to_span().fg(Color::Yellow) + "[Esc]".to_span().fg(Color::Green))
                .alignment(ratatui::layout::HorizontalAlignment::Center),
        )
        .border_type(BorderType::Rounded)
        .bg(Color::default())
        .fg(Color::Cyan);

    frame.render_widget(popup_block, popup_area);

    let [title_area, description_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
            .margin(1)
            .areas(popup_area);

    Paragraph::new(" ".to_span() + item.name.as_str().fg(Color::default()))
        .block(
            Block::bordered()
                .title(" Title ".fg(Color::Yellow))
                .fg(Color::Green)
                .border_type(BorderType::Rounded),
        )
        .render(title_area, frame.buffer_mut());

    Paragraph::new(item.description.as_str().fg(Color::default()))
        .wrap(Wrap { trim: false })
        .block(
            Block::bordered()
                .title(" Description ".fg(Color::Yellow))
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        )
        .render(description_area, frame.buffer_mut());
}

fn render_edit(frame: &mut Frame, app_state: &mut AppState) {
    let area = frame.area();
    let popup_width = (area.width as f32 * 0.3) as u16;
    let popup_height = (area.height as f32 * 0.4) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = ratatui::layout::Rect::new(popup_x, popup_y, popup_width, popup_height);

    let popup_block = Block::bordered()
        .title(
            " Add New Item "
                .to_span()
                .fg(Color::Yellow)
                .into_centered_line(),
        )
        .title_bottom(
            (" Next ".to_span().fg(Color::Yellow)
                + "[Tab]".to_span().fg(Color::Green)
                + " Submit ".to_span().fg(Color::Yellow)
                + "[Enter]".to_span().fg(Color::Green)
                + " Cancel ".to_span().fg(Color::Yellow)
                + "[Esc] ".to_span().fg(Color::Green))
            .alignment(ratatui::layout::HorizontalAlignment::Center),
        )
        .border_type(BorderType::Rounded)
        .fg(Color::Cyan);

    frame.render_widget(popup_block, popup_area);

    let [title_area, description_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
            .margin(1)
            .areas(popup_area);

    Paragraph::new(
        " ".to_span()
            + app_state
                .input_state
                .name_input
                .as_str()
                .fg(Color::default())
            + if app_state.input_state.select_state == InputSelectState::Name {
                "|".to_span().fg(Color::default())
            } else {
                "".to_span()
            },
    )
    .scroll((
        0,
        app_state
            .input_state
            .name_input
            .len()
            .saturating_sub(title_area.width as usize - 4) as u16,
    ))
    .block(
        Block::bordered()
            .title(" Title ".fg(Color::Yellow))
            .fg(
                if app_state.input_state.select_state == InputSelectState::Name {
                    Color::White
                } else {
                    Color::Green
                },
            )
            .border_type(BorderType::Rounded),
    )
    .render(title_area, frame.buffer_mut());

    Paragraph::new(
        app_state
            .input_state
            .description_input
            .as_str()
            .fg(Color::default())
            + if app_state.input_state.select_state == InputSelectState::Description {
                "|".to_span().fg(Color::default())
            } else {
                "".to_span()
            },
    )
    .wrap(Wrap { trim: false })
    .block(
        Block::bordered()
            .title(" Description ".fg(Color::Yellow))
            .fg(
                if app_state.input_state.select_state == InputSelectState::Description {
                    Color::White
                } else {
                    Color::Green
                },
            )
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded),
    )
    .render(description_area, frame.buffer_mut());
}
