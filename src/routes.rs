use serde::Deserialize;
use tide::{Request, Response, Server};

use crate::{commands::InventoryCommand, postgres_storage::get_postgres_engine};

#[derive(Deserialize)]
struct PostSkuBodyRequest {
    action: String,
    qty: i32,
}

async fn post_sku(mut req: Request<()>) -> tide::Result {
    let sku = req.param("sku").unwrap().to_owned();
    let body: PostSkuBodyRequest = req.body_json().await?;
    let mut engine = get_postgres_engine();

    let cmd = match body.action {
        x if x == "add" => InventoryCommand::AddProduct { sku, qty: body.qty },
        x if x == "sell" => InventoryCommand::SellProduct { sku, qty: body.qty },
        _ => {
            let res = Response::new(400);

            return Ok(res);
        }
    };
    let res = match engine.execute(cmd) {
        Ok(_) => Response::new(204),
        Err(_) => Response::new(400),
    };

    Ok(res)
}

pub fn register_routes(app: &mut Server<()>) {
    app.at("/products/:sku").post(post_sku);
}
