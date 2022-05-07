use cqrs_demo::write::routes::register_routes;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let mut app = tide::new();

    register_routes(&mut app);

    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
