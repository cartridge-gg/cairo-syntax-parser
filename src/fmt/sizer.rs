use std::fmt::{Result as FmtResult, Write};

pub struct Sizer {
    size: usize,
}

impl Write for Sizer {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.size += s.len();
        Ok(())
    }
    fn write_char(&mut self, c: char) -> FmtResult {
        self.size += c.len_utf8();
        Ok(())
    }
}

impl Sizer {
    pub fn new() -> Self {
        Self { size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for Sizer {
    fn default() -> Self {
        Self::new()
    }
}
