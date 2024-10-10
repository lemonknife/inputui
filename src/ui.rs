use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::{App, Mode};

pub fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, messages_area] = vertical.areas(frame.area());

    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "i".bold(),
                " to enter insert mode.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Insert => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            Style::default(),
        ),
        _ => (vec![], Style::default()),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.mode {
            Mode::Normal => Style::default().fg(Color::Blue),
            Mode::Insert => Style::default().fg(Color::LightGreen),
            _ => Style::default(),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, input_area);
    match app.mode {
        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        Mode::Normal | Mode::Insert => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            input_area.x + app.column as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        )),
        _ => {}
    }

    let messages: Vec<ListItem> = app
        .message
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::bordered().title("Messages"));
    frame.render_widget(messages, messages_area);
}
