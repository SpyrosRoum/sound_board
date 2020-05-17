mod bot;
mod db;
mod entry;
mod gui;
mod schema;
mod style;
mod word;

#[tokio::main]
async fn main() {
    let pool = db::get_pool().await;
    gui::main(pool);
}
