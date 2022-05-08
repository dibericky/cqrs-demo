use anyhow::{Error, Result};
use postgres::{Client, NoTls};

use crate::{
    envs::get_env,
    events::{EventDetailed, InventoryEventsDetailed},
};

use super::{engine::Engine, storage::Storage};

pub struct PostgresStorage {
    pub client: Client,
}

impl PostgresStorage {
    pub fn new(connstr: &str) -> Result<Self> {
        let client = Client::connect(connstr, NoTls).map_err(anyhow::Error::msg)?;
        Ok(Self { client })
    }
}

impl Storage for PostgresStorage {
    fn get_events(&mut self, key: &str) -> Result<Option<Vec<InventoryEventsDetailed>>> {
        let rows = self.client.query(
            "
        SELECT event_id, sku, qty, inventory_events.event_type_id as event_type_id, description
            FROM inventory_events, event_types
            where
                sku = $1
                and inventory_events.event_type_id = event_types.event_type_id 
            ORDER BY event_id ASC
        ",
            &[&key],
        )?;
        let events = rows
            .iter()
            .map(|item| match item.get(3) {
                "product_sold" => Ok(InventoryEventsDetailed::ProductSold(EventDetailed {
                    id: item.get(0),
                    sku: item.get(1),
                    qty: item.get(2),
                    event_type_id: item.get(3),
                    description: item.get(4),
                })),
                "product_added" => Ok(InventoryEventsDetailed::ProductAdded(EventDetailed {
                    id: item.get(0),
                    sku: item.get(1),
                    qty: item.get(2),
                    event_type_id: item.get(3),
                    description: item.get(4),
                })),
                _ => {
                    let event_type_id: String = item.get(3);
                    return Err(Error::msg(format!(
                        "Unknown event_type_id {}",
                        event_type_id
                    )));
                }
            })
            .collect::<Result<Vec<InventoryEventsDetailed>>>()?;
        if events.is_empty() {
            return Ok(None);
        }
        Ok(Some(events))
    }
}
pub fn get_postgres_engine() -> Engine {
    let connstr = get_env("POSTGRES_CONN_STRING");
    let postgres = PostgresStorage::new(&connstr).expect("Unable to connect to event storage");
    Engine::new(Box::new(postgres))
}
