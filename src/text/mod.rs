pub struct Source {
    pub text: String,
}

impl Source {
    pub const fn new(text: String) -> Self {
        Self { text }
    }

    pub fn line_index(&self, position: usize) -> usize {
        self.text[..position].lines().count() - 1
    }

    pub fn get_line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap_or_default()
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text
            .lines()
            .take(index)
            .map(|line| line.len() + 1)
            .sum()
    }
}
