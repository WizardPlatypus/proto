#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    pretty_env_logger::init();

    log::info!("Launching...");
    let _rocket = proto::build::build().launch().await?;

    Ok(())
}
