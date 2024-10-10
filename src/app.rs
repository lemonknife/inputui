use std::ops::Range;

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
        }
    }

    pub fn column_add(&self, amount: usize) -> usize {
        let new_column = self.column.saturating_add(amount);
        self.clamp_index(new_column)
    }

    pub fn column_sub(&self, amount: usize) -> usize {
        let new_column = self.column.saturating_sub(amount);
        self.clamp_index(new_column)
    }

    pub fn move_left(&mut self) {
        self.column = self.column_sub(1);
    }

    pub fn move_right(&mut self) {
        self.column = self.column_add(1);
    }

    pub fn clamp_index(&self, new_column: usize) -> usize {
        new_column.clamp(0, self.input.chars().count())
    }

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

    pub fn insert_text(&mut self, input: char) {
        let index = self.byte_index();

        // Iterate over the characters of the input
        self.input.insert(index, input);
        self.move_right();
    }

    pub fn submit_message(&mut self) {
        self.message.push(self.input.clone());
        self.input.clear();
        self.column = 0;
    }
}
