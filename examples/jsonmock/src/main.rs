use sofie::{
    App, Response,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")).init();

    let mut app = App::default();

    app.serve(|req| async move { Ok(Response::builder().text("Hello World")) })
        .await?;

    Ok(())
}
