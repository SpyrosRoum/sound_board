mod bot;
mod db;
mod entry;
mod gui;

// use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let pool = db::get_pool().await;
    gui::main(pool);
}
