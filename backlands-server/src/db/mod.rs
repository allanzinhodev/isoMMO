use sqlx::MySqlPool;
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Player {
    pub id:       u32,
    pub name:     String,
    pub vocation: i8,
    pub looktype: i8,
    pub pos_x:    i16,
    pub pos_y:    i16,
}

pub async fn create_pool(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("failed to connect to MySQL")
}

pub async fn run_migrations(pool: &MySqlPool) {
    sqlx::raw_sql(include_str!("../../schema.sql"))
        .execute(pool)
        .await
        .expect("failed to run schema migrations");
}

pub async fn seed_default_account(pool: &MySqlPool) {
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounts WHERE id = 1")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    if exists.0 == 0 {
        let hash = hash_password("backlands123");
        sqlx::query("INSERT INTO accounts (id, name, password) VALUES (1, 'admin', ?)")
            .bind(&hash)
            .execute(pool)
            .await
            .expect("failed to seed account");

        sqlx::query(
            "INSERT INTO players (account_id, name, vocation, looktype) VALUES
             (1, 'Ze Cangaceiro', 0, 0),
             (1, 'Maria Curandeira', 1, 1),
             (1, 'Frei Azulao', 2, 2)"
        )
        .execute(pool)
        .await
        .expect("failed to seed players");

        println!("[db] seeded default account (admin/backlands123) with 3 characters");
    }
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("hash failed")
        .to_string()
}

pub async fn verify_account(pool: &MySqlPool, username: &str, password: &str) -> Option<u32> {
    let row: Option<(u32, String)> =
        sqlx::query_as("SELECT id, password FROM accounts WHERE name = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
            .ok()?;

    let (id, hash) = row?;
    let parsed = PasswordHash::new(&hash).ok()?;
    Argon2::default().verify_password(password.as_bytes(), &parsed).ok()?;
    Some(id)
}

pub async fn get_players(pool: &MySqlPool, account_id: u32) -> Vec<Player> {
    sqlx::query_as(
        "SELECT id, name, vocation, looktype, pos_x, pos_y FROM players WHERE account_id = ?"
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn get_player(pool: &MySqlPool, player_id: u32, account_id: u32) -> Option<Player> {
    sqlx::query_as(
        "SELECT id, name, vocation, looktype, pos_x, pos_y FROM players WHERE id = ? AND account_id = ?"
    )
    .bind(player_id)
    .bind(account_id)
    .fetch_optional(pool)
    .await
    .ok()?
}

pub async fn save_player_pos(pool: &MySqlPool, player_id: u32, x: i16, y: i16) {
    let _ = sqlx::query("UPDATE players SET pos_x = ?, pos_y = ? WHERE id = ?")
        .bind(x)
        .bind(y)
        .bind(player_id)
        .execute(pool)
        .await;
}
