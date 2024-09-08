mod worker_func;
mod db;
fn main() {
    db::init_db();
    worker_func::start_worker();
}
