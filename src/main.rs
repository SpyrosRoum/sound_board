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
    db::create_tables(&pool).await;
    let words = db::get_words(&pool).await;

    gui::main(pool, words);
}
