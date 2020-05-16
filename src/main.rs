mod bot;
mod db;
mod entry;
mod gui;
mod schema;
mod style;

#[tokio::main]
async fn main() {
    let pool = db::get_pool().await;
    gui::main(pool);
}
