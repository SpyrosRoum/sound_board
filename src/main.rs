mod db;

use async_std::task;

fn main() -> Result<(), sqlx::Error> {
    let pool = task::block_on(db::create_sqlite_pool())?;
    task::block_on(db::create_db_tables(&pool));

    println!("cool");
    Ok(())
}
