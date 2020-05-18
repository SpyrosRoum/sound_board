mod bot;
mod db;
mod entry;
mod gui;
mod schema;
mod style;
mod word;

use std::thread;
use std::sync::mpsc::channel;

use tokio::runtime::Runtime;


fn main() {
    let (sx1, rx1) = channel();
    let (sx2, rx2) = channel();

    thread::spawn(move || {
        let mut rt = Runtime::new().unwrap();

        rt.block_on(async {
            let pool = db::get_pool().await;
            db::create_tables(&pool).await;
            let words = db::get_words(&pool).await;

            sx1.send(pool).unwrap();
            sx2.send(words).unwrap();
        });
    });

    let pool = rx1.recv().unwrap();
    let words = rx2.recv().unwrap();

    gui::main(pool, words);
}
