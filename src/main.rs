use cqrs_demo::commands::InventoryCommand;
use cqrs_demo::events::InventoryEvents;
use cqrs_demo::{postgres_storage::PostgresStorage, engine::Engine};
use dotenv::dotenv;

use tide::Body;
use tide::Request;
use tide::Response;
use tide::prelude::*;

fn get_postgres_engine () -> Engine {
    let connstr = get_env("POSTGRES_CONN_STRING");
    let postgres = PostgresStorage::new(&connstr).expect("Unable to connect to event storage");
    Engine::new(Box::new(postgres))
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let mut app = tide::new();

    app
        .at("/products/:sku/")
        .get(get_sku);
    app
        .at("/products/:sku/events/")
        .get(get_sku_events);

    // app
    //     .at("/products/:sku")
    //     .get(post_sku);

    app.listen("127.0.0.1:3000").await?;
    Ok(())
}

#[derive(Serialize)]
struct InventoryEventRaw {
    id: i32,
    action: String,
    qty: i32,
    sku: String
}

impl From<InventoryEvents> for InventoryEventRaw {
    fn from(event: InventoryEvents) -> Self {
        let action = event.get_id();
        match event {
            InventoryEvents::ProductAdded { sku, qty, id } => Self{ action, sku, qty, id: id.unwrap() },
            InventoryEvents::ProductSold { sku, qty , id} => Self{ action, sku, qty , id: id.unwrap() },
        }
    }
}

async fn get_sku_events(req: Request<()>) -> tide::Result {
    let sku = req.param("sku").unwrap().to_owned();
    let mut engine = get_postgres_engine();
    let events = engine.get_events(&sku)?;
    let events : Vec<InventoryEventRaw> = events.unwrap_or_default()
        .into_iter()
        .map(|event| event.into())
        .collect();
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&events)?);
    Ok(res)
}

async fn get_sku(req: Request<()>) -> tide::Result {
    let sku = req.param("sku").unwrap().to_owned();
    let mut engine = get_postgres_engine();
    let product = engine.get_product(&sku);

    let res = match product {
        None => {
            let res = Response::new(404);
            res
        },
        Some(product) => {
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&product)?);
            res
        }
    };
   
    Ok(res)
}

#[derive(Deserialize)]
struct PostSkyBodyRequest {
    action: String,
    qty: i32
}

// async fn post_sku(req: Request<()>) -> tide::Result {
//     let sku = req.param("sku").unwrap().to_owned();
//     let mut engine = get_postgres_engine();
//     let body: PostSkyBodyRequest = req.body_json().await?;

    // let cmd = match body.action {
    //     x if x == "add" => InventoryCommand::AddProduct {
    //         sku,
    //         qty: body.qty,
    //     },
    //     x if x == "sell" => InventoryCommand::SellProduct { sku, qty: body.qty },
    //     _ => {
    //         let res = Response::new(403);
   
    //         return Ok(res)
    //     }
    // };
    // let _result = engine.execute(cmd);
//     let res = Response::new(204);
   
//     Ok(res)
// }

fn get_env(env_name: &str) -> String {
    std::env::var(env_name).ok().expect(&format!("{} must be set", env_name))
}