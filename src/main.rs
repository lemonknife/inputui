use std::io::stderr;

use app::{App, Mode};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::queue;
use ratatui::DefaultTerminal;

mod app;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let app_result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    app_result
}

fn set_cursor_block() -> Result<()> {
    use crossterm::cursor::SetCursorStyle;
    Ok(queue!(stderr(), SetCursorStyle::SteadyBlock)?)
}

fn set_cursor_bar() -> Result<()> {
    use crossterm::cursor::SetCursorStyle;
    Ok(queue!(stderr(), SetCursorStyle::SteadyBar)?)
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    set_cursor_block()?;
    loop {
        terminal.draw(|frame| ui::ui(frame, app))?;
        if app.exit {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            match app.mode {
                // TODO: complete Normal Mode
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => app.exit = true,
                    KeyCode::Char('i') => {
                        app.mode = Mode::Insert;
                        set_cursor_bar()?;
                    }
                    KeyCode::Left => app.move_left(),
                    KeyCode::Right => app.move_right(),
                    _ => {}
                },
                Mode::Insert if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(value) => app.insert_text(value),
                    KeyCode::Backspace => app.remove_char((app.column_sub(1), app.column)),
                    KeyCode::Left => app.move_left(),
                    KeyCode::Right => app.move_right(),
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                        set_cursor_block()?;
                    }
                    _ => {}
                },
                Mode::Insert => {}
                // TODO: add other modes
                _ => {}
            }
        }
    }
}
