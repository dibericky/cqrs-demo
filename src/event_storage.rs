use crate::events::InventoryEvents;

pub trait EventStorage {
    fn add_event(&mut self, key: &str, event: InventoryEvents);
    fn exists(&self, key: &str) -> bool;
    fn get_events(&self, key: &str) -> Option<Vec<InventoryEvents>>;
}
