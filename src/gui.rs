use std::thread;

use super::bot;

use iui::controls::{Button, Entry, Group, HorizontalBox, Label, PasswordEntry, VerticalBox};
use iui::prelude::*;

pub struct Gui {
    pub ui: UI,
    pub error_label: Label,
}

impl Gui {
    pub fn new() -> Self {
        let ui = UI::init().expect("Failed to initialize the UI library");
        let error_label = Label::new(&ui, "");
        Gui {
            ui,
            error_label
        }
    }

    pub fn build_gui(&mut self) {
        // let ui = &mut self.ui;
        let mut win = Window::new(&self.ui, "Sound Board", 640, 480, WindowType::NoMenubar);

        // Create groups
        let mut words_group = Group::new(&self.ui, "Words");
        let mut bot_group = Group::new(&self.ui, "Bot");

        // Create boxes
        // The main box
        let mut vbox = VerticalBox::new(&self.ui);
        vbox.set_padded(&self.ui, true);
        // Fox the bot group
        let mut hbox = HorizontalBox::new(&self.ui);
        hbox.set_padded(&self.ui, true);

        // Set up Controls for the box group
        let mut e_token = PasswordEntry::new(&self.ui);
        e_token.set_value(&self.ui, "Token");

        let mut btn_bot = Button::new(&self.ui, "Start Bot");
        btn_bot.on_clicked(&self.ui, |_| {
            self.start_bot(e_token.value(&self.ui));
        });

        // Add controls to the box
        hbox.append(&self.ui, btn_bot.clone(), LayoutStrategy::Compact);
        hbox.append(&self.ui, e_token.clone(), LayoutStrategy::Stretchy);
        bot_group.set_child(&self.ui, hbox);

        vbox.append(&self.ui, bot_group, LayoutStrategy::Compact);

        // A label for error messages
        vbox.append(&self.ui, self.error_label.clone(), LayoutStrategy::Stretchy);

        win.set_child(&self.ui, vbox);

        win.show(&self.ui);
    }

    pub fn print_error(&mut self, err_msg: &str) {
        self.error_label.set_text(&self.ui, err_msg);
    }

    fn start_bot(&self, token: String) {
        // If there is a panic  there will be no feedback..
        thread::spawn(|| {
            bot::start(token);
        });
    }
}