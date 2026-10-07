use facelock_common::error::Result;
use rde_facelockd::app::App;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut app = App::new()?;

    if let Err(e) = app.start().await {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
