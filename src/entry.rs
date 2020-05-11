use iced::{
    button, text_input, Align, Button, Element,
    Length, Row, Text, TextInput,
};

#[derive(Debug, Clone)]
pub struct Entry {
    word: String,
    id: String,
    path: String,

    state: EntryState,
}

#[derive(Debug, Clone)]
enum EntryState {
    Idle {
        edit_btn: button::State,
    },
    Editing {
        word_in: text_input::State,
        id_in: text_input::State,
        path_btn: button::State,
        done_btn: button::State,
        delete_btn: button::State,
    },
}

impl Default for EntryState {
    fn default() -> Self {
        Self::Editing {
            word_in: text_input::State::new(),
            id_in: text_input::State::new(),
            path_btn: button::State::new(),
            done_btn: button::State::new(),
            delete_btn: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryMessage {
    ChooseFile,
    // ChoseFile(String),
    WordChanged(String),
    IdChanged(String),
    Edit,
    DoneEditing,
    Delete,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            id: String::new(),
            path: String::from("Path"),
            state: EntryState::default(),
        }
    }

    pub fn update(&mut self, message: EntryMessage) {
        match message {
            EntryMessage::WordChanged(new) => self.word = new,
            EntryMessage::IdChanged(new) => self.id = new,
            EntryMessage::Edit => {
                self.state = EntryState::Editing {
                    word_in: text_input::State::new(),
                    id_in: text_input::State::new(),
                    path_btn: button::State::new(),
                    done_btn: button::State::new(),
                    delete_btn: button::State::new(),
                }
            }
            EntryMessage::DoneEditing => {
                // TODO check that id is all numbers
                if !self.word.is_empty() && !self.path.is_empty() && !self.id.is_empty() {
                    self.state = EntryState::Idle {
                        edit_btn: button::State::new(),
                    }
                }
            }
            // This is taken care of in gui.rs
            EntryMessage::Delete => {}
            EntryMessage::ChooseFile => {} //return Command::perform(choose_file(), Message::ChoseFile),
            // EntryMessage::ChoseFile(path) => {
            //     // TODO this is for testing only for now
            //     if path != "-1" {
            //         println!("{}", path);
            //     } else {
            //         println!("Cancelled")
            //     }
            // }
        }
    }

    pub fn view(&mut self) -> Element<EntryMessage> {
        match &mut self.state {
            EntryState::Idle { edit_btn } => {
                let word_lbl = Text::new(&self.word);
                let id_lbl = Text::new(&self.id);
                let path_lbl = Text::new(&self.path);
                let edit_btn = Button::new(edit_btn, Text::new("edit"))
                    .on_press(EntryMessage::Edit)
                    .padding(10);

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(word_lbl)
                    .push(id_lbl)
                    .push(path_lbl)
                    .push(edit_btn)
                    .into()
            }
            EntryState::Editing {
                word_in,
                id_in,
                path_btn,
                done_btn,
                delete_btn,
            } => {
                let word = TextInput::new(
                    word_in,
                    "Word",
                    &self.word,
                    EntryMessage::WordChanged,
                )
                .padding(20)
                .width(Length::Fill);

                let id = TextInput::new(
                    id_in,
                    "Channel Id",
                    &self.id,
                    EntryMessage::IdChanged,
                )
                .padding(20)
                .width(Length::Fill);

                let path = Button::new(path_btn, Text::new(&self.path))
                    .on_press(EntryMessage::ChooseFile)
                    .padding(10);

                let done = Button::new(done_btn, Text::new("Done"))
                    .on_press(EntryMessage::DoneEditing)
                    .padding(10);
                let delete = Button::new(delete_btn, Text::new("Delete"))
                    .on_press(EntryMessage::Delete)
                    .padding(10);

                Row::new()
                    .spacing(20)
                    .push(word)
                    .push(id)
                    .push(path)
                    .push(done)
                    .push(delete)
                    .into()
            }
        }
    }
}
