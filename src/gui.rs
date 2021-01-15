use std::sync::{Arc, Mutex};

use super::black_word::{BlackWordEntry, BlackWordMessage};
use super::bot;
use super::db;
use super::entry::{Entry, EntryMessage};
use super::style::Theme;
use super::word::Word;

use iced::{
    button, scrollable, text_input, Align, Application, Button, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Space, Text, TextInput,
};
use sqlx::SqlitePool;

pub fn main(pool: SqlitePool, words: Vec<Word>, blackwords: Vec<String>) {
    SoundBoard::run(Settings::with_flags((pool, words, blackwords)));
}

struct SoundBoard {
    style: Theme,
    message: Arc<Mutex<String>>,
    bot_running: bool,
    start_bot_btn: button::State,
    save_btn: button::State,
    token: text_input::State,
    add_entry_btn: button::State,
    add_black_word_btn: button::State,
    token_value: String,
    scroll_entries: scrollable::State,
    scroll_black_entries: scrollable::State,

    connection_pool: Arc<Mutex<SqlitePool>>,
    words: Arc<Mutex<Vec<Word>>>,
    entries: Vec<Entry>,
    blacklist: Vec<String>,
    blacklist_entries: Vec<BlackWordEntry>,
}

impl SoundBoard {
    fn new(pool: SqlitePool, words: Vec<Word>, blackwords: Vec<String>) -> Self {
        Self {
            style: Theme::Dark,
            message: Arc::new(Mutex::new(String::new())),
            bot_running: false,
            start_bot_btn: button::State::default(),
            save_btn: button::State::default(),
            token: text_input::State::new(),
            add_entry_btn: button::State::default(),
            add_black_word_btn: Default::default(),
            token_value: String::new(),
            scroll_entries: scrollable::State::new(),
            scroll_black_entries: scrollable::State::new(),
            connection_pool: Arc::new(Mutex::new(pool)),
            words: Arc::new(Mutex::new(words)),
            entries: Vec::new(),
            blacklist: blackwords,
            blacklist_entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GotToken(String),
    GotEntries(Vec<Entry>),
    GotBlackEntries(Vec<BlackWordEntry>),
    StartBotPressed,
    TokenChanged(String),
    BotFailed,
    Save,
    Saved,
    AddEntry,
    AddBlackWord,
    EntryMessage(usize, EntryMessage),
    BlackWordMessage(usize, BlackWordMessage),
    NewWords(Vec<Word>),
    NewBlackWords(Vec<String>),
}

impl Application for SoundBoard {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (SqlitePool, Vec<Word>, Vec<String>);

    fn new(
        (pool, words, blackwords): (SqlitePool, Vec<Word>, Vec<String>),
    ) -> (Self, Command<Self::Message>) {
        (
            Self::new(pool, words, blackwords),
            Command::perform(db::get_token(), Message::GotToken),
        )
    }

    fn title(&self) -> String {
        String::from("Sound Board")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::EntryMessage(i, EntryMessage::Delete) => {
                self.entries.remove(i);
            }
            Message::EntryMessage(i, msg) => {
                if let Some(entry) = self.entries.get_mut(i) {
                    return entry
                        .update(msg)
                        .map(move |msg| Message::EntryMessage(i, msg));
                }
            }
            Message::BlackWordMessage(i, BlackWordMessage::Delete) => {
                self.blacklist_entries.remove(i);
            }
            Message::BlackWordMessage(i, msg) => {
                if let Some(black_word) = self.blacklist_entries.get_mut(i) {
                    return black_word
                        .update(msg)
                        .map(move |msg| Message::BlackWordMessage(i, msg));
                }
            }
            Message::GotToken(token) => {
                if !token.starts_with("Bot") {
                    self.token_value = token;
                }
                return Command::perform(
                    db::get_entries(Arc::clone(&self.connection_pool)),
                    Message::GotEntries,
                );
            }
            Message::GotEntries(entries) => {
                self.entries = entries;
                return Command::perform(
                    db::get_blacklist_entries(Arc::clone(&self.connection_pool)),
                    Message::GotBlackEntries,
                );
            }
            Message::GotBlackEntries(entries) => self.blacklist_entries = entries,
            Message::StartBotPressed => {
                if self.bot_running {
                    let mut lbl = self.message.lock().unwrap();
                    *lbl = "Bot is already running".to_string();
                } else {
                    let mut lbl = self.message.lock().unwrap();
                    *lbl = "Starting Bot".to_string();
                    self.bot_running = true;
                    return Command::perform(
                        start_bot(
                            self.token_value.clone(),
                            Arc::clone(&self.words),
                            self.blacklist_entries
                                .iter()
                                .map(|b| b.word.to_string())
                                .collect(),
                            Arc::clone(&self.message),
                        ),
                        |_| Message::BotFailed,
                    );
                }
            }
            Message::TokenChanged(new) => {
                self.token_value = new;
            }
            Message::Save => {
                return Command::perform(
                    db::save(
                        Arc::clone(&self.connection_pool),
                        self.token_value.clone(),
                        self.entries.clone(),
                        self.blacklist_entries.clone(),
                    ),
                    |_| Message::Saved,
                );
            }
            Message::Saved => {
                let mut lbl = self.message.lock().unwrap();
                *lbl = "Saved".to_string();
                return Command::perform(
                    db::get_new_words(Arc::clone(&self.connection_pool)),
                    Message::NewWords,
                );
            }
            Message::NewWords(new_words) => {
                let mut words = self.words.lock().unwrap();
                *words = new_words;
                return Command::perform(
                    db::get_new_blacklist(Arc::clone(&self.connection_pool)),
                    Message::NewBlackWords,
                );
            }
            Message::NewBlackWords(new_words) => self.blacklist = new_words,
            Message::AddEntry => {
                let index = self.entries.len();
                let entry = Entry::new(index);
                self.entries.push(entry);
            }
            Message::AddBlackWord => {
                let index = self.blacklist.len();
                let entry = BlackWordEntry::new(index);
                self.blacklist_entries.push(entry);
            }
            Message::BotFailed => {
                let mut lbl = self.message.lock().unwrap();
                *lbl = "Failed to start the bot. Make sure you have the correct token".to_string();
                self.bot_running = false;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let add_entry = Button::new(&mut self.add_entry_btn, Text::new("Add Entry"))
            .on_press(Message::AddEntry)
            .padding(20)
            .style(self.style);

        let add_black_word = Button::new(
            &mut self.add_black_word_btn,
            Text::new("Add Black Listed Word"),
        )
        .on_press(Message::AddBlackWord)
        .padding(20)
        .style(self.style);

        let lbl = self.message.lock().unwrap();
        let messages_lbl = Text::new(lbl.clone()).size(20);

        let bot_btn = Button::new(&mut self.start_bot_btn, Text::new("Start Bot"))
            .on_press(Message::StartBotPressed)
            .padding(20)
            .style(self.style);

        let token_input = TextInput::new(
            &mut self.token,
            "Bot Token",
            &self.token_value,
            Message::TokenChanged,
        )
        .password()
        .padding(20)
        .style(self.style);

        let save_btn = Button::new(&mut self.save_btn, Text::new("Save"))
            .on_press(Message::Save)
            .padding(20)
            .style(self.style);

        let head = Row::new()
            .spacing(20)
            .push(Text::new("Word"))
            .push(Text::new("Channel Id"))
            .push(Text::new("Sound file"))
            .padding(20);

        let black_list_head = Row::new()
            .spacing(20)
            .push(Text::new("Black list"))
            .padding(20);

        let entries: Element<_> = if self.entries.len() > 0 {
            self.entries
                .iter_mut()
                .enumerate()
                .fold(Column::new().spacing(20), |col, (i, entry)| {
                    col.push(
                        entry
                            .view()
                            .map(move |message| Message::EntryMessage(i, message)),
                    )
                })
                .align_items(Align::Center)
                .into()
        } else {
            Text::new("You don't have any words")
                .width(Length::Fill)
                .size(25)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into()
        };

        let black_entries: Element<_> = if self.blacklist_entries.len() > 0 {
            self.blacklist_entries
                .iter_mut()
                .enumerate()
                .fold(Column::new().spacing(20), |col, (i, entry)| {
                    col.push(
                        entry
                            .view()
                            .map(move |msg| Message::BlackWordMessage(i, msg)),
                    )
                })
                .align_items(Align::Center)
                .into()
        } else {
            Text::new("You don't have any blacklisted words")
                .width(Length::Fill)
                .size(25)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into()
        };

        Container::new(
            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(head)
                .push(
                    Scrollable::new(&mut self.scroll_entries)
                        .spacing(5)
                        .push(entries)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                        .max_height(450)
                        .padding(20)
                        .align_items(Align::Center),
                )
                .push(add_entry)
                .push(Space::with_height(Length::Units(50)))
                .push(black_list_head)
                .push(
                    Scrollable::new(&mut self.scroll_black_entries)
                        .spacing(5)
                        .push(black_entries)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                        .max_height(450)
                        .padding(20)
                        .align_items(Align::Center),
                )
                .push(add_black_word)
                .push(
                    Row::new()
                        .padding(20)
                        .push(bot_btn)
                        .push(token_input)
                        .align_items(Align::Center),
                )
                .push(save_btn)
                .push(Space::with_height(Length::Fill))
                .push(messages_lbl)
                .align_items(Align::Center),
        )
        .style(self.style)
        .into()
    }
}

async fn start_bot(
    token: String,
    words: Arc<Mutex<Vec<Word>>>,
    black_list: Vec<String>,
    msg: Arc<Mutex<String>>,
) {
    bot::start(token, words, black_list, msg).await;
}
