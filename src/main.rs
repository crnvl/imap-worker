use tokio;

mod worker_func;
mod db;

#[tokio::main]
async fn main() {
    db::init_db().await;
    worker_func::start_worker().await;
}
