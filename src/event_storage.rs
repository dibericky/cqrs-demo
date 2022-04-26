use anyhow::Result;

use crate::events::InventoryEvents;

pub trait EventStorage {
    fn add_event(&mut self, key: &str, event: InventoryEvents) -> Result<()>;
    fn exists(&mut self, key: &str) -> Result<bool>;
    fn get_events(&mut self, key: &str) -> Result<Option<Vec<InventoryEvents>>>;
}
