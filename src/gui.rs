use super::bot;
use super::db;

use iced::{
    button, scrollable, text_input, Align, Application, Button, Column, Command, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Space, Text, TextInput,
};
use nfd;
use tokio::task;

pub fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    message: String,
    bot_running: bool,
    start_bot_btn: button::State,
    save_btn: button::State,
    token: text_input::State,
    choose_file_btn: button::State,
    token_value: String,
    scroll: scrollable::State,

    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
enum Message {
    // Good,
    Saved(bool),
    CreatedTables,
    GotToken(String),
    StartBotPressed,
    TokenChanged(String),
    BotFailed,
    Save,
    ChooseFile,
    ChoseFile(String),
    AddEntry,
}

#[derive(Debug, Clone)]
struct Entry {
    word: String,
    id: String,
    path: String,

    state: EntryState,
}

#[derive(Debug, Clone)]
struct EntryState {
    word: text_input::State,
    id: text_input::State,
    path: button::State,
}

impl Default for EntryState {
    fn default() -> Self {
        EntryState {
            word: text_input::State::new(),
            id: text_input::State::new(),
            path: button::State::new(),
        }
    }
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::perform(db::create_tables(), |_| Message::CreatedTables),
        )
    }

    fn title(&self) -> String {
        String::from("Sound Board")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            // Message::Good => (),
            Message::CreatedTables => {
                return Command::perform(db::get_token(), Message::GotToken);
            }
            Message::GotToken(token) => {
                if !token.starts_with("Bot") {
                    self.token_value = token;
                }
            }
            Message::StartBotPressed => {
                if self.bot_running {
                    self.message = "Bot is already running".to_string();
                } else {
                    self.message = "Starting Bot".to_string();
                    self.bot_running = true;
                    return Command::perform(start_bot(self.token_value.clone()), |_| {
                        // If this runs, `start_bot` finished.
                        // FIXME This never actually runs even if bot panics
                        Message::BotFailed
                    });
                }
            }
            Message::TokenChanged(new) => {
                self.token_value = new;
            }
            Message::Save => {
                return Command::perform(db::save(self.token_value.clone()), Message::Saved);
            }
            Message::Saved(success) => {
                self.message = if success {
                    "Saved".to_string()
                } else {
                    "Error Saving".to_string()
                };
            }
            Message::ChooseFile => return Command::perform(choose_file(), Message::ChoseFile),
            Message::ChoseFile(path) => {
                // TODO this is for testing only for now
                if path != "-1" {
                    println!("{}", path);
                } else {
                    println!("Cancelled")
                }
            }
            Message::AddEntry => {
                let entry = Entry {
                    word: "Word".to_string(),
                    id: self.entries.len().to_string(),
                    path: "path".to_string(),
                    state: EntryState::default(),
                };
                self.entries.push(entry);
            }
            Message::BotFailed => {
                self.message =
                    "Failed to start the bot. Make sure you have the correct token".to_string();
                self.bot_running = false;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let choose_file_btn = Button::new(&mut self.choose_file_btn, Text::new("Test"))
            .on_press(Message::AddEntry)
            .padding(20);

        let messages_lbl = Text::new(self.message.clone()).size(20);

        let bot_btn = Button::new(&mut self.start_bot_btn, Text::new("Start Bot"))
            .on_press(Message::StartBotPressed)
            .padding(20);

        let token_input = TextInput::new(
            &mut self.token,
            "Bot Token",
            &self.token_value,
            Message::TokenChanged,
        )
        .password()
        .padding(20);

        let save_btn = Button::new(&mut self.save_btn, Text::new("Save"))
            .on_press(Message::Save)
            .padding(20);

        let head = Row::new()
            .spacing(20)
            .push(Text::new("Word"))
            .push(Text::new("Channel Id"))
            .push(Text::new("Sound file"))
            .padding(20);
        
        let entries: Element<_> = if self.entries.len() > 0 {
            self.entries
                .iter_mut()
                .fold(Column::new().spacing(20), |col, entry| {
                    col.push(
                        Row::new()
                            .spacing(20)
                            .push(Text::new(&entry.word))
                            .push(Text::new(&entry.id))
                            .push(Text::new(&entry.path)),
                    )
                })
                .into()
        } else {
            Text::new("You don't have any words")
                .width(Length::Fill)
                .size(25)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into()
        };

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(head)
            .push(
                Scrollable::new(&mut self.scroll)
                    .spacing(5)
                    .align_items(Align::Center)
                    .push(entries)
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .max_height(200)
                    .padding(20),
            )
            .push(choose_file_btn)
            .push(Space::with_height(Length::Units(50)))
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(bot_btn)
                    .push(token_input),
            )
            .push(save_btn)
            .push(Space::with_height(Length::Fill))
            .push(messages_lbl)
            .into()
    }
}

async fn start_bot(token: String) {
    bot::start(token).await;
}

// I can't return the nfd::Response (doesn't impl Debug) so I use -1 when I want to ignore it
async fn choose_file() -> String {
    (task::spawn_blocking(|| {
        let res = nfd::open_file_dialog(None, None).expect("Error opening nfd");

        match res {
            nfd::Response::Okay(path) => path,
            nfd::Response::Cancel => "-1".to_string(),
            _ => "-1".to_string(),
        }
    })
    .await)
        .unwrap()
}
