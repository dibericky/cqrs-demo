use cqrs_demo::{read, write};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let mut app = tide::new();

    write::routes::register(&mut app);
    read::routes::register(&mut app);

    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
