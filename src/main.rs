use app::App;
use color_eyre::Result;
use crossterm::event::{self, Event};
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

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    App::set_cursor_block()?;
    loop {
        terminal.draw(|frame| ui::ui(frame, app))?;
        if app.exit {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            app.handle_key(key)?;
        }
    }
}
