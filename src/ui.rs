use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::app::{App, Mode};

pub fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1), // Info
        Constraint::Length(3), // Input
        Constraint::Min(1),    // Message
        Constraint::Length(1), // Status bar
    ]);
    let [help_area, input_area, messages_area, status_area] = vertical.areas(frame.area());

    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "i".bold(),
                " to enter insert mode.".bold(),
            ],
            Style::default(),
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
            input_area.x + app.input[..app.byte_index()].width() as u16 + 1,
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

    let status_left = vec![
        match &app.mode {
            Mode::Normal => {
                Span::styled(" NORMAL", Style::default().fg(Color::Black).bg(Color::Blue))
            }
            Mode::Insert => Span::styled(
                " INSERT",
                Style::default().fg(Color::Black).bg(Color::LightGreen),
            ),
            _ => Span::styled("      ", Style::default()),
        },
        match &app.mode {
            Mode::Normal => Span::styled("", Style::default().bg(Color::Black).fg(Color::Blue)),
            Mode::Insert => {
                Span::styled("", Style::default().bg(Color::Black).fg(Color::LightGreen))
            }
            _ => Span::styled("", Style::default().bg(Color::Black)),
        },
    ];
    let status_right = vec![
        Span::styled(
            &app.key_pressed,
            Style::default().fg(Color::Magenta).bg(Color::Black),
        ),
        match &app.mode {
            Mode::Normal => {
                Span::styled(" ", Style::default().bg(Color::Black).fg(Color::DarkGray))
            }
            Mode::Insert => {
                Span::styled(" ", Style::default().bg(Color::Black).fg(Color::DarkGray))
            }
            _ => Span::styled(" ", Style::default().bg(Color::Black)),
        },
        match &app.mode {
            Mode::Normal => Span::styled(
                " Baka Input ",
                Style::default().bg(Color::DarkGray).fg(Color::Blue),
            ),
            Mode::Insert => Span::styled(
                " Baka Input ",
                Style::default().bg(Color::DarkGray).fg(Color::LightGreen),
            ),
            _ => Span::styled(" Baka Input ", Style::default().bg(Color::DarkGray)),
        },
        match &app.mode {
            Mode::Normal => Span::styled("", Style::default().bg(Color::DarkGray).fg(Color::Blue)),
            Mode::Insert => Span::styled(
                "",
                Style::default().bg(Color::DarkGray).fg(Color::LightGreen),
            ),
            _ => Span::styled("", Style::default().bg(Color::DarkGray)),
        },
        match &app.mode {
            Mode::Normal => Span::styled(
                "Cirno ",
                Style::default().fg(Color::DarkGray).bg(Color::Blue),
            ),
            Mode::Insert => Span::styled(
                "Cirno ",
                Style::default().fg(Color::DarkGray).bg(Color::LightGreen),
            ),
            _ => Span::styled(
                "Cirno ",
                Style::default().fg(Color::DarkGray).bg(Color::Blue),
            ),
        },
    ];

    let [status_left_area, status_mid_area, status_right_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(get_content_length(&status_left)),
            Constraint::Min(0),
            Constraint::Length(get_content_length(&status_right)),
        ])
        .areas(status_area);

    let status_left =
        Paragraph::new(Line::from(status_left)).block(Block::default().borders(Borders::NONE));
    let status_right =
        Paragraph::new(Line::from(status_right)).block(Block::default().borders(Borders::NONE));
    let status_mid = Paragraph::default().block(
        Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Black)),
    );

    frame.render_widget(status_left, status_left_area);
    frame.render_widget(status_right, status_right_area);
    frame.render_widget(status_mid, status_mid_area);
}

fn get_content_length(spans: &[Span]) -> u16 {
    spans
        .iter()
        .map(|span| span.content.chars().count())
        .sum::<usize>() as u16
}
