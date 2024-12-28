mod app;
mod command;
mod handler;

#[tokio::main]
async fn main() {
  match app::run().await {
    Ok(_) => (),
    Err(err) => eprintln!("Error: {:?}", err),
  }
}
