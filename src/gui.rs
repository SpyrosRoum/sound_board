use super::bot;
use super::db;

use iced::{
    button, text_input, Align, Application, Button, Column, Command, Element, Row, Settings, Text,
    TextInput, Space, Length,
};
use sqlx::SqlitePool;

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
    token_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    // Good,
    CreatedPool(SqlitePool),
    CreatedTables(SqlitePool),
    GotToken(String),
    StartBotPressed,
    TokenChanged(String),
    BotFailed,
    Save,
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::perform(
                db::create_pool("sqlite://DATA/app.db"),
                Message::CreatedPool,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Sound Board")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            // Message::Good => (),
            Message::CreatedPool(pool) => {
                return Command::perform(db::create_tables(pool.clone()), Message::CreatedTables);
            }
            Message::CreatedTables(pool) => {
                return Command::perform(db::get_token(pool.clone()), Message::GotToken);
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
                        // This never actually runs even if bot panics
                        Message::BotFailed
                    });
                }
            }
            Message::TokenChanged(new) => {
                self.token_value = new;
            }
            Message::Save => {
                self.message = "Saved".to_string();
            }
            // If this get's sent it means that the bot is not running anymore.
            // Probably because of panic/wrong token
            Message::BotFailed => {
                // TODO say wrong token
                println!("Probably wrong token");
                self.bot_running = false;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
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
