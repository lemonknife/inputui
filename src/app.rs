use std::io::stderr;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::{execute, queue};
pub enum Mode {
    Normal,
    Operate(Operation),
    Insert,
}

pub enum Operation {
    /// Neovim 'd' behavior
    Delete(OperateState),

    /// Neovim 'c'
    Change(OperateState),
}

pub enum OperateState {
    /// For better escape use
    Deactive,
    Active,
    /// For regular operation use: delete inner part
    Inner,
    /// For regular operation use: delete outer part
    Outer,
}

pub struct App {
    /// Input Mode
    pub mode: Mode,
    /// Used for better escape behavior
    pub escape_state: OperateState,
    /// Input String
    pub input: String,
    /// Whether we exit the App
    pub exit: bool,
    /// Cursor Index
    pub column: usize,
    /// Total Message
    pub message: Vec<String>,
    /// Detect Key Pressed
    pub key_pressed: String,
}

impl App {
    pub const fn new() -> Self {
        Self {
            mode: Mode::Normal,
            escape_state: OperateState::Deactive,
            input: String::new(),
            exit: false,
            column: 0,
            message: Vec::new(),
            key_pressed: String::new(),
        }
    }

    /// Set Cursor to Block (for normal mode)
    pub fn set_cursor_block() -> Result<()> {
        use crossterm::cursor::SetCursorStyle;
        Ok(queue!(stderr(), SetCursorStyle::SteadyBlock)?)
    }

    /// Set Cursor to bar (for insert mode)
    pub fn set_cursor_bar() -> Result<()> {
        use crossterm::cursor::SetCursorStyle;
        Ok(queue!(stderr(), SetCursorStyle::SteadyBar)?)
    }

    /// Helper function for savely add column addition with some value
    ///
    /// # Arguments
    ///
    /// * `amount`: column will add amount
    ///
    /// TODO: reload addition
    fn column_add(&self, amount: usize, open: bool) -> usize {
        let new_column = self.column.saturating_add(amount);
        self.clamp_index(new_column, open)
    }

    /// Helper function for savely subtract column with some value
    ///
    /// # Arguments
    ///
    /// * `amount`: column will minus amount
    ///
    /// TODO: reload subtraction
    fn column_sub(&self, amount: usize, open: bool) -> usize {
        let new_column = self.column.saturating_sub(amount);
        self.clamp_index(new_column, open)
    }

    /// Save movement to left for cursor
    ///
    /// # Arguments
    ///
    /// * `amount`: the amount of position move to left
    pub fn move_left(&mut self, amount: usize, open: bool) {
        self.column = self.column_sub(amount, open);
    }

    /// Save movement to right for cursor
    ///
    /// # Arguments
    ///
    /// * `amount`: the amount of position move to right
    /// * `open`: if it is in a open interval
    pub fn move_right(&mut self, amount: usize, open: bool) {
        self.column = self.column_add(amount, open);
    }

    /// Helper Function to restrict the range of cursor
    ///
    /// # Arguments
    ///
    /// * `new_column`: the value which needs restrict
    /// * `open`: if it is in a open interval
    ///
    /// Range of cursor in Insert Mode: `0..=chars_of_input`
    /// Range of cursor in Normal/Operation Mode: ``0..chars_of_input``
    fn clamp_index(&self, new_column: usize, open: bool) -> usize {
        new_column.clamp(0, self.input.chars().count().saturating_sub(open as usize))
    }

    /// Calculate the index of a character in &str
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.column)
            .unwrap_or(self.input.len())
    }

    /// Remove the char
    ///
    /// # Arguments
    ///
    /// * `start`: start removing characters
    /// * `end`: end remove characters (not include end_index character)
    ///
    /// # Examples
    /// For backspace key:
    /// ```
    /// self.remove_char((self.column_sub(1)), self.column))
    /// ```
    /// For delete key:
    /// ```
    /// self.remove_char((self.column, self.column_add(1)))
    /// ```
    /// For deleting range:
    /// ```
    /// // "123" -> "3"
    /// // start_index == 0
    /// // end_index == 1
    /// self.remove_char((start_index, end_index + 1))
    /// ```
    pub fn remove_char(&mut self, (start, end): (usize, usize)) {
        if start != end {
            let left = self.input.chars().take(start);
            let right = self.input.chars().skip(end);

            self.input = left.chain(right).collect();

            // TODO: need validation
            self.column = start;
        }
    }

    pub fn insert_text(&mut self, input: &str, prepend: bool) {
        self.move_right(prepend as usize, false);
        let index = self.byte_index();

        // Iterate over the characters of the input
        self.input.insert_str(index, input);
        self.move_right(input.chars().count() - 1, true);
        if let Mode::Insert = self.mode {
            self.move_right(1, false);
        };
    }

    pub fn submit_message(&mut self) {
        self.message.push(self.input.clone());
        self.input.clear();
        self.column = 0;
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        self.key_pressed = key.code.to_string();
        match self.mode {
            // TODO: complete Normal Mode
            Mode::Normal => match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                    Self::set_cursor_bar()?;
                }
                KeyCode::Char('a') => {
                    self.mode = Mode::Insert;
                    self.move_right(1, false);
                    Self::set_cursor_bar()?;
                }
                KeyCode::Left | KeyCode::Char('h') => self.move_left(1, true),
                KeyCode::Right | KeyCode::Char('l') => self.move_right(1, true),
                KeyCode::Char('p') => self.insert_text("Baka琪露诺", true),
                KeyCode::Char('P') => self.insert_text("Baka琪露诺", false),
                _ => {}
            },
            Mode::Insert if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => self.submit_message(),
                KeyCode::Char(value) => self.insert_text(value.to_string().as_str(), false),
                KeyCode::Backspace => self.remove_char((self.column_sub(1, false), self.column)),
                KeyCode::Left => self.move_left(1, false),
                KeyCode::Right => self.move_right(1, false),
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.move_left(1, true);
                    Self::set_cursor_block()?;
                }
                _ => {}
            },
            Mode::Insert => {}
            // TODO: add other modes
            _ => {}
        };
        Ok(())
    }
}
