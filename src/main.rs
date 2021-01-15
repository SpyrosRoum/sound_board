#![windows_subsystem = "windows"]

mod black_word;
mod bot;
mod db;
mod entry;
mod gui;
mod schema;
mod style;
mod word;

use std::sync::mpsc::channel;
use std::thread;

use tokio::runtime::Runtime;

fn main() {
    let (sx1, rx1) = channel();
    let (sx2, rx2) = channel();
    let (sx3, rx3) = channel();

    thread::spawn(move || {
        let mut rt = Runtime::new().unwrap();

        rt.block_on(async {
            let pool = db::get_pool().await;
            db::create_tables(&pool).await;
            let words = db::get_words(&pool).await;
            let blackwords = db::get_blacklist(&pool).await;

            sx1.send(pool).unwrap();
            sx2.send(words).unwrap();
            sx3.send(blackwords).unwrap();
        });
    });

    let pool = rx1.recv().unwrap();
    let words = rx2.recv().unwrap();
    let blackwords = rx3.recv().unwrap();

    gui::main(pool, words, blackwords);
}
