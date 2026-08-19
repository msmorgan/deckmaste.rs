use crate::context::ParseContext;

pub trait Render {
    fn render(
        &self,
        context: &ParseContext<'_>,
        environment: &crate::environment::ParserEnvironment,
    ) -> String;
}

pub(crate) struct Writer {
    output: String,
    capitalize_next: bool,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
        }
    }

    pub(crate) fn word(&mut self, word: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        if self.capitalize_next {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
            self.capitalize_next = false;
        } else {
            self.output.push_str(word);
        }
    }

    pub(crate) fn identity(&mut self, identity: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        self.output.push_str(identity);
        self.capitalize_next = false;
    }

    pub(crate) fn punctuation(&mut self, mark: char) {
        self.output.push(mark);
        self.capitalize_next = mark == '.';
    }

    pub(crate) fn finish(self) -> String {
        self.output
    }
}
