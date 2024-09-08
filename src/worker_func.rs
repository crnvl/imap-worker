use std::thread;

use sqlx::{Pool, Postgres};

use crate::db::{self, IterableIP};

pub fn ip_by_iterator(ip_index: i64) -> String {
    let ip = format!(
        "{}.{}.{}.{}",
        ip_index / 16777216,
        (ip_index / 65536) % 256,
        (ip_index / 256) % 256,
        ip_index % 256
    );
    ip
}

async fn worker_thread(client: Pool<Postgres>, id: i64, from: i64, to: i64) {
    let mut ip_index = from;

    loop {
        if ip_index == to {
            ip_index = from;
        }

        let ip = ip_by_iterator(ip_index);
        ping_to_ip(client.clone(), id, &ip, &ip_index).await;

        ip_index += 1;
    }
}

pub async fn start_worker() {
    let db_handle = db::db_get_handle().await;
    println!("worker manager received handle");

    let from: i64 = 0;
    let to: i64 = 4294967296;
    let thread_count: i64 = 1024;

    let range = to - from;
    let range_per_thread = range / thread_count;

    let mut handles = vec![];

    for i in 0..thread_count {
        let from = from + i * range_per_thread;
        let to = from + range_per_thread;

        let from = from.clone();
        let to = to.clone();

        let db_handle_clone = db_handle.clone();
        let handle = thread::spawn(move || {
            println!("[worker-{}] thread started: {} - {}", i, from, to);
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(worker_thread(db_handle_clone, i, from, to));
            println!("[worker-{}]  thread finished: {} - {}", i, from, to);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn ping_to_ip(client: Pool<Postgres>, id: i64, ip: &String, ip_index: &i64) {
    let data = [0; 0];
    let timeout = std::time::Duration::from_secs(1);

    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };

    let parsed_ip: std::net::IpAddr = ip.as_str().parse().unwrap();
    let result = ping_rs::send_ping(&parsed_ip, timeout, &data, Some(&options));

    match result {
        Ok(reply) => {
            println!("[worker-{}] successful reply from {:?}", id, reply.address);

            let latency: i64 = reply.rtt as i64;
            db::insert_ip(
                id,
                client,
                IterableIP {
                    id: ip_index.to_owned(),
                    ip: ip.to_owned(),
                    latency: latency,
                    online: true,
                },
            )
            .await;
        }
        Err(_) => {
            db::insert_ip(
                id,
                client,
                IterableIP {
                    id: ip_index.to_owned(),
                    ip: ip.to_owned(),
                    latency: 0,
                    online: false,
                },
            )
            .await
        }
    }
}
