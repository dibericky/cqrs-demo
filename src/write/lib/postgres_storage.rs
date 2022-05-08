use anyhow::{Error, Result};
use postgres::{Client, NoTls};

use crate::{envs::get_env, events::InventoryEvents};

use super::{engine::Engine, event_storage::EventStorage};

pub struct PostgresStorage {
    pub client: Client,
}

impl PostgresStorage {
    pub fn new(connstr: &str) -> Result<Self> {
        let client = Client::connect(connstr, NoTls).map_err(anyhow::Error::msg)?;
        Ok(Self { client })
    }
}

impl EventStorage for PostgresStorage {
    fn add_event(&mut self, key: &str, event: InventoryEvents) -> Result<()> {
        let qty = match event {
            InventoryEvents::ProductAdded { qty, .. } => qty,
            InventoryEvents::ProductSold { qty, .. } => qty,
        };
        let event_type_id = event.get_id();
        self.client.execute(
            "INSERT INTO inventory_events (event_type_id, sku, qty) VALUES ($1, $2, $3)",
            &[&event_type_id, &key, &qty],
        )?;
        Ok(())
    }

    fn exists(&mut self, key: &str) -> Result<bool> {
        let res = self.client.query(
            "SELECT event_id FROM inventory_events WHERE sku = $1 LIMIT 1",
            &[&key],
        )?;
        Ok(!res.is_empty())
    }

    fn get_events(&mut self, key: &str) -> Result<Option<Vec<InventoryEvents>>> {
        let rows = self.client.query("SELECT event_type_id, sku, qty, event_id FROM inventory_events WHERE sku = $1 ORDER BY event_id ASC", &[&key])?;
        let events = rows
            .iter()
            .map(|item| match item.get(0) {
                "product_sold" => Ok(InventoryEvents::ProductSold {
                    sku: item.get(1),
                    qty: item.get(2),
                }),
                "product_added" => Ok(InventoryEvents::ProductAdded {
                    sku: item.get(1),
                    qty: item.get(2),
                }),
                _ => {
                    let sku: String = item.get(0);
                    return Err(Error::msg(format!("Unknown event_type_id {}", sku)));
                }
            })
            .collect::<Result<Vec<InventoryEvents>>>()?;
        if events.is_empty() {
            return Ok(None);
        }
        Ok(Some(events))
    }
}

pub fn get_postgres_engine() -> Engine {
    let connstr = get_env("POSTGRES_CONN_STRING");
    let postgres = PostgresStorage::new(&connstr).expect("Unable to connect to event storage");
    Engine::new(Box::new(postgres), None)
}
