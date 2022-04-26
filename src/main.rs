
use anyhow::Result;
use cqrs_demo::{postgres_storage::PostgresStorage, engine::Engine, commands::InventoryCommand};
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();
    let connstr = get_env("POSTGRES_CONN_STRING");
    let postgres = PostgresStorage::new(&connstr).expect("Unable to connect to event storage");

    let mut engine = Engine::new(Box::new(postgres));

    let cmd = InventoryCommand::AddProduct {
        sku: "abc".to_string(),
        qty: 5,
    };
    let _ = engine.execute(cmd)?;

    let cmd = InventoryCommand::SellProduct {
        sku: "abc".to_string(),
        qty: 2,
    };
    let _ = engine.execute(cmd)?;
    
    let product = engine.get_product("abc").unwrap();

    println!("Res {:?}", product);
    drop(engine);

    Ok(())
}

fn get_env(env_name: &str) -> String {
    std::env::var(env_name).ok().expect(&format!("{} must be set", env_name))
}