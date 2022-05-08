use anyhow::Result;

use crate::events::InventoryEventsDetailed;

pub trait Storage {
    fn get_events(&mut self, key: &str) -> Result<Option<Vec<InventoryEventsDetailed>>>;
}
