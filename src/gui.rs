use std::thread;

use super::bot;

use iui::controls::{Button, Entry, Group, HorizontalBox, Label, PasswordEntry, VerticalBox};
use iui::prelude::*;

pub fn create_gui() -> UI {
    let ui = UI::init().expect("Failed to initialize the UI library");

    let mut win = Window::new(&ui, "Sound Board", 640, 480, WindowType::NoMenubar);

    // Create groups
    let mut words_group = Group::new(&ui, "Words");
    let mut bot_group = Group::new(&ui, "Bot");

    // Create boxes
    // The main box
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);
    // Fox the bot group
    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);

    // Set up Controls for the box group
    let mut e_token = PasswordEntry::new(&ui);
    e_token.set_value(&ui, "Token");

    let mut btn_bot = Button::new(&ui, "Start Bot");
    btn_bot.on_clicked(&ui, |_| {
        start_bot(e_token.value(&ui));
    });

    // Add controls to the box
    hbox.append(&ui, btn_bot.clone(), LayoutStrategy::Compact);
    hbox.append(&ui, e_token.clone(), LayoutStrategy::Stretchy);
    bot_group.set_child(&ui, hbox);

    vbox.append(&ui, bot_group, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);

    win.show(&ui);

    ui
}

fn start_bot(token: String) {
    println!("Starting");
    thread::spawn(|| {
        bot::start(token);
    });
    println!("done");
}
