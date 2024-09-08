use std::thread;

use crate::db::{self, IterableIP};

pub fn ip_by_iterator(ip_index: u64) -> String {
    let ip = format!(
        "{}.{}.{}.{}",
        ip_index / 16777216,
        (ip_index / 65536) % 256,
        (ip_index / 256) % 256,
        ip_index % 256
    );
    ip
}

fn worker_thread(id: u64, from: u64, to: u64) {
    let mut ip_index = from;
    loop {
        if ip_index == to {
            ip_index = from;
        }

        let ip = ip_by_iterator(ip_index);

        // ping ip asynchronously
        let ip_cpy = ip.clone();

        thread::spawn(move || {
            let ip_index = ip_index.clone();
            let ip_cpy = ip_cpy.clone();
            let _ = ping_to_ip(id, &ip_cpy, &ip_index);
        });

        if ip_index % 10000000 == 0 {
            println!("[worker-{}]: {}", id, ip);
        }
        ip_index += 1;
    }
}

pub fn start_worker() {
    let from = 0;
    let to = 4294967296;
    let thread_count: u64 = 12;

    let range = to - from;
    let range_per_thread = range / thread_count;

    let mut handles = vec![];

    for i in 0..thread_count {
        let from = from + i * range_per_thread;
        let to = from + range_per_thread;

        let from = from.clone();
        let to = to.clone();

        let handle = thread::spawn(move || {
            println!("[worker-{}] thread started: {} - {}", i, from, to);
            worker_thread(i, from, to);
            println!("[worker-{}]  thread finished: {} - {}", i, from, to);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn ping_to_ip(id: u64, ip: &String, ip_index: &u64) {
    println!("[worker-{}] Pinging: {}", id, ip);

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
            println!("[worker-{}] Reply: {:?}", id, reply);

            let latency = reply.rtt;
            db::insert_ip(IterableIP {
                id: ip_index.to_owned(),
                ip: ip.to_owned(),
                latency: latency,
                online: true,
            })
        }
        Err(_) => db::insert_ip(IterableIP {
            id: ip_index.to_owned(),
            ip: ip.to_owned(),
            latency: 0,
            online: false,
        }),
    }
}
