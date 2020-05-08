mod bot;
mod db;
mod gui;

use gui::Gui;

use tokio::runtime::Runtime;

fn main() -> Result<(), sqlx::Error> {
    // Create new scope for a runtime that can then be dropped
    // the pool will be dropped too but we don't mind since we will create a new one when starting the bot
    {
        let mut rt = Runtime::new().expect("Failed to create runtime 1");
        rt.block_on(async {
            let pool = db::create_sqlite_pool()
                .await
                .expect("Failed to create pool for db 1");
            db::create_db_tables(&pool).await;
        })
    }

    let mut gui = Gui::new();
    gui.build_gui();

    gui.ui.main();
    Ok(())
}
