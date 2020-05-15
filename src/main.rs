mod bot;
mod db;
mod entry;
mod gui;


#[tokio::main]
async fn main() {
    let pool = db::get_pool().await;
    gui::main(pool);
}
