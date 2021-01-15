#[derive(Debug, Clone)]
pub struct Word {
    pub word: String,
    pub chn_id: String,
    pub path: String,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            word: String::new(),
            chn_id: String::new(),
            path: String::from("Path"),
        }
    }
}

impl Word {
    fn is_empty(&self) -> bool {
        self.word.is_empty() || self.path.is_empty() || self.chn_id.is_empty()
    }

    /// Determine if the channel id related to the word is numeric
    fn id_numeric(&self) -> bool {
        self.chn_id.chars().all(char::is_numeric)
    }

    pub fn is_valid(&self) -> bool {
        !self.is_empty() && self.id_numeric()
    }
}
