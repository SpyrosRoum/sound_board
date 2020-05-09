use std::sync::mpsc;
use std::thread;

use super::bot;
use super::db;

use iced::{
    button, text_input, Align, Application, Button, Column, Command, Element, Length, Row,
    Settings, Space, Text, TextInput,
};
use nfd;


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
            Message::ChooseFile => {
                // TODO things..
                let (sx, rx) = mpsc::channel();
                thread::spawn(move || {
                    let res = nfd::open_file_dialog(None, None).unwrap();
                    sx.send(res).unwrap();
                });
                let res = rx.recv().unwrap();
                match res {
                    nfd::Response::Okay(path) => println!("{}", path),
                    nfd::Response::Cancel => println!("Cancelled"),
                    _ => (),
                }
            }
            Message::BotFailed => {
                self.message = "Failed to start the bot. Make sure you have the correct token".to_string();
                self.bot_running = false;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let choose_file_btn = Button::new(&mut self.choose_file_btn, Text::new("Test"))
            .on_press(Message::ChooseFile)
            .padding(20);

        let messages_lbl = Text::new(self.message.clone()).size(35);

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

        Column::new()
            .padding(20)
            .align_items(Align::Center)
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
