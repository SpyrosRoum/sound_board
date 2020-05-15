use iced::{button, text_input, Align, Button, Element, Length, Row, Text, TextInput};
use nfd;

#[derive(Debug, Clone)]
pub struct Entry {
    index: usize,

    pub word: String,
    pub g_id: String,
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
        g_id_in: text_input::State,
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
            g_id_in: text_input::State::new(),
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
    GuildIdChanged(String),
    ChnIdChanged(String),
    Edit,
    DoneEditing,
    Delete,
    ChoseFile(String),
}

impl Entry {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            word: String::new(),
            g_id: String::new(),
            chn_id: String::new(),
            path: String::from("Path"),
            state: EntryState::default(),
        }
    }

    pub fn new_idle(index: usize) -> Self {
        Self {
            index,
            word: String::new(),
            g_id: String::new(),
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
            EntryMessage::GuildIdChanged(new) => self.g_id = new,
            EntryMessage::ChnIdChanged(new) => self.chn_id = new,
            EntryMessage::Edit => {
                self.state = EntryState::Editing {
                    word_in: text_input::State::new(),
                    chn_id_in: text_input::State::new(),
                    g_id_in: text_input::State::new(),
                    path_btn: button::State::new(),
                    done_btn: button::State::new(),
                    delete_btn: button::State::new(),
                }
            }
            EntryMessage::DoneEditing => {
                // TODO check that id is all numbers
                if !self.word.is_empty()
                    && !self.path.is_empty()
                    && (!self.g_id.is_empty() || !self.chn_id.is_empty())
                {
                    self.state = EntryState::Idle {
                        edit_btn: button::State::new(),
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
                let g_id_lbl = Text::new(&self.g_id);
                let chn_id_lbl = Text::new(&self.chn_id);
                let path_lbl = Text::new(&self.path);
                let edit_btn = Button::new(edit_btn, Text::new("edit"))
                    .on_press(EntryMessage::Edit)
                    .padding(10);

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(word_lbl)
                    .push(g_id_lbl)
                    .push(chn_id_lbl)
                    .push(path_lbl)
                    .push(edit_btn)
                    .into()
            }
            EntryState::Editing {
                word_in,
                chn_id_in,
                g_id_in,
                path_btn,
                done_btn,
                delete_btn,
            } => {
                let word = TextInput::new(word_in, "Word", &self.word, EntryMessage::WordChanged)
                    .padding(20)
                    .width(Length::Fill);

                let chn_id = TextInput::new(
                    chn_id_in,
                    "Channel Id",
                    &self.chn_id,
                    EntryMessage::ChnIdChanged,
                )
                .padding(20)
                .width(Length::Fill);

                let g_id = TextInput::new(
                    g_id_in,
                    "Guild Id",
                    &self.g_id,
                    EntryMessage::GuildIdChanged,
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
                    .push(g_id)
                    .push(chn_id)
                    .push(path)
                    .push(done)
                    .push(delete)
                    .into()
            }
        }
    }
}
