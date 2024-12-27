mod app;

#[tokio::main]
async fn main() {
  match app::run().await {
    Ok(_) => (),
    Err(err) => eprintln!("Error: {:?}", err),
  }
}
