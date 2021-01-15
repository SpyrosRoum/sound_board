use super::style::Theme;

use iced::{button, text_input, Align, Button, Command, Element, Length, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub struct BlackWordEntry {
    style: Theme,
    index: usize,

    pub word: String,

    state: BlackWordState,
}

#[derive(Debug, Clone)]
enum BlackWordState {
    Idle {
        edit_btn: button::State,
    },
    Editing {
        word_in: text_input::State,
        done_btn: button::State,
        delete_btn: button::State,
    },
}

impl Default for BlackWordState {
    fn default() -> Self {
        Self::Editing {
            word_in: text_input::State::new(),
            done_btn: button::State::new(),
            delete_btn: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlackWordMessage {
    WordChanged(String),
    Edit,
    DoneEditing,
    Delete,
}

impl BlackWordEntry {
    pub fn new(index: usize) -> Self {
        Self {
            style: Theme::Dark,
            index,
            word: String::new(),
            state: BlackWordState::default(),
        }
    }

    pub fn new_idle(index: usize) -> Self {
        Self {
            style: Theme::Dark,
            index,
            word: String::new(),
            state: BlackWordState::Idle {
                edit_btn: button::State::new(),
            },
        }
    }

    pub fn update(&mut self, message: BlackWordMessage) -> Command<BlackWordMessage> {
        match message {
            // This is taken care of in gui.rs
            BlackWordMessage::Delete => {}
            BlackWordMessage::WordChanged(new) => self.word = new.to_lowercase(),
            BlackWordMessage::Edit => self.state = BlackWordState::default(),
            BlackWordMessage::DoneEditing => {
                if !self.word.is_empty() {
                    self.state = BlackWordState::Idle {
                        edit_btn: button::State::new(),
                    }
                }
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<BlackWordMessage> {
        match &mut self.state {
            BlackWordState::Idle { edit_btn } => {
                let word_lbl = Text::new(&self.word);
                let edit_btn = Button::new(edit_btn, Text::new("edit"))
                    .on_press(BlackWordMessage::Edit)
                    .padding(10)
                    .style(self.style);

                Row::new()
                    .spacing(20)
                    .push(word_lbl)
                    .push(edit_btn)
                    .align_items(Align::Center)
                    .into()
            }
            BlackWordState::Editing {
                word_in,
                done_btn,
                delete_btn,
            } => {
                let word =
                    TextInput::new(word_in, "Word", &self.word, BlackWordMessage::WordChanged)
                        .padding(20)
                        .width(Length::Fill)
                        .style(self.style);

                let done = Button::new(done_btn, Text::new("Done"))
                    .on_press(BlackWordMessage::DoneEditing)
                    .padding(10)
                    .style(self.style);
                let delete = Button::new(delete_btn, Text::new("Delete"))
                    .on_press(BlackWordMessage::Delete)
                    .padding(10)
                    .style(self.style);

                Row::new()
                    .spacing(20)
                    .push(word)
                    .push(done)
                    .push(delete)
                    .align_items(Align::Center)
                    .into()
            }
        }
    }
}
