use super::bot;
use super::db;

use sqlx::SqlitePool;

use iced::{
    button, text_input, Align, Button, Column, Element, Row, Application, Settings, Text, TextInput, Command,
};

pub fn main() {
    Counter::run(Settings::default())
}

struct Counter {
    pool: Option<SqlitePool>,
    bot_running: bool,
    start_bot_btn: button::State,
    token: text_input::State,
    token_value: String,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            pool: None,
            bot_running: false,
            start_bot_btn: Default::default(),
            token: Default::default(),
            token_value: "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    CreateDb(SqlitePool),
    StartBotPressed,
    TokenChanged(String),
    BotFailed,
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::perform(create_db(), Message::CreateDb))
    }

    fn title(&self) -> String {
        String::from("Sound Board")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::CreateDb(pool) => {
                self.pool = Some(pool);
            },
            Message::StartBotPressed => {
                if self.bot_running {
                    // TODO say it's running
                    println!("It's running ")
                } else {
                    self.bot_running = true;
                    return Command::perform(start_bot(self.token_value.clone()), |_| Message::BotFailed);
                }
            }
            Message::TokenChanged(new) => {
                self.token_value = new;
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
            .into()
    }
}

// async fn get_token() -> String {
// }

async fn create_db() -> SqlitePool {
    let pool = db::create_pool().await;
    db::create_tables(&pool).await;
    pool
}

async fn start_bot(token: String) {
    bot::start(token).await;
}