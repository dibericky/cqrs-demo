use tide::{Body, Request, Response, Server};

use crate::events::EventDetailed;

use super::lib::postgres::get_postgres_engine;

async fn get_events(req: Request<()>) -> tide::Result {
    let sku = req.param("sku").unwrap().to_owned();
    let mut engine = get_postgres_engine();
    let events = engine.get_events(&sku);

    let res = match events {
        None => Response::new(404),
        Some(events) => {
            let mut res = Response::new(200);
            let events_detailed: Vec<EventDetailed> =
                events.into_iter().map(|event| event.into()).collect();
            res.set_body(Body::from_json(&events_detailed)?);
            res
        }
    };
    Ok(res)
}

pub fn register(app: &mut Server<()>) {
    app.at("/products/:sku/events").get(get_events);
}
