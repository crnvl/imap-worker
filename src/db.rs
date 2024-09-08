pub struct IterableIP {
    pub id: u64,
    pub ip: String,
    pub latency: u32,
    pub online: bool,
}

pub fn init_db() -> sqlite::Connection {
    let connection = sqlite::open("db.sqlite").unwrap();

    create_table();
    println!("Database initialized");

    connection
}

pub fn db_get_handle() -> sqlite::Connection {
    sqlite::open("db.sqlite").unwrap()
}

pub fn create_table() {
    let connection = db_get_handle();
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS iterable_ip (
                id INTEGER PRIMARY KEY,
                ip TEXT NOT NULL,
                latency INTEGER NOT NULL,
                online BOOLEAN NOT NULL
            )
            ",
        )
        .unwrap();
}

pub fn insert_ip(ip: IterableIP) {
    let connection = db_get_handle();

    let statement = format!(
        "
        INSERT OR REPLACE INTO iterable_ip (id, ip, latency, online)
        VALUES ({}, '{}', {}, {})
        ",
        ip.id, ip.ip, ip.latency, ip.online
    );

    connection.execute(&statement).unwrap();
}
