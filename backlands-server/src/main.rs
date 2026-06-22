mod db;
mod game;
mod network;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let server_addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:7171".into());

    let pool = db::create_pool(&database_url).await;
    println!("[db] connected to MySQL");

    db::run_migrations(&pool).await;
    println!("[db] schema ready");

    db::seed_default_account(&pool).await;

    network::run(pool, &server_addr).await;
}
