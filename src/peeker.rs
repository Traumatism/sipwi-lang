/// Get next/previous values in a vector
pub struct Peeker<T> {
    pub values: Vec<T>,
    pub cursor: usize,
}

impl<T: Clone> Peeker<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self { values, cursor: 0 }
    }

    /// Get the previous value
    pub fn previous(&mut self) -> Option<T> {
        self.cursor -= 2;
        self.next()
    }

    /// Get the next value
    pub fn next(&mut self) -> Option<T> {
        if self.cursor >= self.values.len() {
            return None;
        }

        let value = self.values[self.cursor].to_owned();
        self.cursor += 1;

        Some(value)
    }
}
