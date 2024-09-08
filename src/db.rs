use sqlx::{Executor, Pool, Postgres};

pub struct IterableIP {
    pub id: i64,
    pub ip: String,
    pub latency: i64,
    pub online: bool,
}

pub async fn init_db() {
    create_table().await;
    println!("Database initialized");
}

pub async fn db_get_handle() -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10000)
        .connect("postgres://imap:imap@localhost:5432")
        .await
        .unwrap()
}

pub async fn create_table() {
    let client = db_get_handle().await;

    client
        .execute(
            "
        CREATE TABLE IF NOT EXISTS iterable_ip (
            id BIGINT PRIMARY KEY,
            ip TEXT NOT NULL,
            latency BIGINT NOT NULL,
            online BOOLEAN NOT NULL
        )
        ",
        )
        .await
        .unwrap();
}

pub async fn insert_ip(id: i64, client: Pool<Postgres>, ip: IterableIP) {
    // insert or replace
    let statement = format!(
        "
        INSERT INTO iterable_ip (id, ip, latency, online)
        VALUES ({}, '{}', {}, {})
        ON CONFLICT (id) DO UPDATE SET
        ip = '{}', latency = {}, online = {}
        ",
        ip.id, ip.ip, ip.latency, ip.online, ip.ip, ip.latency, ip.online
    );

    let result = client.execute(statement.as_str()).await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("[worker-{}] error inserting ip: {}", id, e);
        }
    }
}
