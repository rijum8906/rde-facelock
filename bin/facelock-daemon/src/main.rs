use facelock_daemon::app::App;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut app = App::new();

    if let Err(e) = app.start().await {
        eprintln!("Error: {}", e);
    }
}
