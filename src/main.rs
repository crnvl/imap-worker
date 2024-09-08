use tokio;

mod worker_func;
mod db;

#[tokio::main]
async fn main() {
    let now = std::time::Instant::now();

    db::init_db().await;
    worker_func::start_worker().await;

    println!("Elapsed time: {:?}", now.elapsed());
}
