use std::path::Path;

use super::style::Theme;

use iced::{button, text_input, Align, Button, Element, Length, Row, Text, TextInput};
use nfd;

#[derive(Debug, Clone)]
pub struct Entry {
    style: Theme,
    index: usize,

    pub word: String,
    pub chn_id: String,
    pub path: String,

    state: EntryState,
}

#[derive(Debug, Clone)]
enum EntryState {
    Idle {
        edit_btn: button::State,
    },
    Editing {
        word_in: text_input::State,
        chn_id_in: text_input::State,
        path_btn: button::State,
        done_btn: button::State,
        delete_btn: button::State,
    },
}

impl Default for EntryState {
    fn default() -> Self {
        Self::Editing {
            word_in: text_input::State::new(),
            chn_id_in: text_input::State::new(),
            path_btn: button::State::new(),
            done_btn: button::State::new(),
            delete_btn: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryMessage {
    ChooseFile,
    WordChanged(String),
    ChnIdChanged(String),
    Edit,
    DoneEditing,
    Delete,
    ChoseFile(String),
}

impl Entry {
    pub fn new(index: usize) -> Self {
        Self {
            style: Theme::Dark,
            index,
            word: String::new(),
            chn_id: String::new(),
            path: String::from("Path"),
            state: EntryState::default(),
        }
    }

    pub fn new_idle(index: usize) -> Self {
        Self {
            style: Theme::Dark,
            index,
            word: String::new(),
            chn_id: String::new(),
            path: String::from("Path"),
            state: EntryState::Idle {
                edit_btn: button::State::new(),
            },
        }
    }

    pub fn update(&mut self, message: EntryMessage) {
        match message {
            // This is taken care of in gui.rs
            EntryMessage::Delete => {}
            EntryMessage::WordChanged(new) => self.word = new.to_lowercase(),
            EntryMessage::ChnIdChanged(new) => self.chn_id = new,
            EntryMessage::Edit => {
                self.state = EntryState::Editing {
                    word_in: text_input::State::new(),
                    chn_id_in: text_input::State::new(),
                    path_btn: button::State::new(),
                    done_btn: button::State::new(),
                    delete_btn: button::State::new(),
                }
            }
            EntryMessage::DoneEditing => {
                if !self.word.is_empty() && !self.path.is_empty() && !self.chn_id.is_empty() {
                    if self.chn_id.chars().all(char::is_numeric) {
                        self.state = EntryState::Idle {
                            edit_btn: button::State::new(),
                        }
                    }
                }
            }
            EntryMessage::ChooseFile => {
                let res = nfd::open_file_dialog(None, None).expect("Error opening nfd");
                match res {
                    nfd::Response::Okay(path) => self.update(EntryMessage::ChoseFile(path)),
                    _ => (),
                };
            }
            EntryMessage::ChoseFile(path) => {
                println!("{}", path);
                self.path = path;
            }
        }
    }

    pub fn view(&mut self) -> Element<EntryMessage> {
        match &mut self.state {
            EntryState::Idle { edit_btn } => {
                let word_lbl = Text::new(&self.word);
                let chn_id_lbl = Text::new(&self.chn_id);
                let file_name = Path::new(&self.path).file_name().unwrap();
                let path_lbl = Text::new(file_name.to_string_lossy());
                let edit_btn = Button::new(edit_btn, Text::new("edit"))
                    .on_press(EntryMessage::Edit)
                    .padding(10)
                    .style(self.style);

                Row::new()
                    .spacing(20)
                    .push(word_lbl)
                    .push(chn_id_lbl)
                    .push(path_lbl)
                    .push(edit_btn)
                    .align_items(Align::Center)
                    .into()
            }
            EntryState::Editing {
                word_in,
                chn_id_in,
                path_btn,
                done_btn,
                delete_btn,
            } => {
                let word = TextInput::new(word_in, "Word", &self.word, EntryMessage::WordChanged)
                    .padding(20)
                    .width(Length::Fill)
                    .style(self.style);

                let chn_id = TextInput::new(
                    chn_id_in,
                    "Channel Id",
                    &self.chn_id,
                    EntryMessage::ChnIdChanged,
                )
                .padding(20)
                .width(Length::Fill)
                    .style(self.style);

                let file_name = Path::new(&self.path).file_name().unwrap();
                let path = Button::new(path_btn, Text::new(file_name.to_string_lossy()))
                    .on_press(EntryMessage::ChooseFile)
                    .padding(10)
                    .style(self.style);

                let done = Button::new(done_btn, Text::new("Done"))
                    .on_press(EntryMessage::DoneEditing)
                    .padding(10)
                    .style(self.style);
                let delete = Button::new(delete_btn, Text::new("Delete"))
                    .on_press(EntryMessage::Delete)
                    .padding(10)
                    .style(self.style);

                Row::new()
                    .spacing(20)
                    .push(word)
                    .push(chn_id)
                    .push(path)
                    .push(done)
                    .push(delete)
                    .align_items(Align::Center)
                    .into()
            }
        }
    }
}
