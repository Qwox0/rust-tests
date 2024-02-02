use std::time::Duration;
use tokio;

const SLEEP: Duration = Duration::from_secs(2);

async fn wait() {
    tokio::time::sleep(SLEEP).await;
}

fn tokio_runtime() {
    println!("before tokio_runtime");
    let rt = tokio::runtime::Builder::new_multi_thread().enable_time().build().unwrap();
    rt.block_on(async {
        wait().await;
    });
    println!("after  tokio_runtime");
}

fn thread_sleep() {
    println!("before thread_sleep");
    std::thread::sleep(SLEEP);
    println!("after  thread_sleep");
}

fn main() {
    tokio_runtime();
    thread_sleep();
}
