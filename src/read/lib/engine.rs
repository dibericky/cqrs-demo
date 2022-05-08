use crate::events::InventoryEventsDetailed;

use super::storage::Storage;

pub struct Engine {
    storage: Box<dyn Storage>,
}

impl Engine {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn get_events(&mut self, sku: &str) -> Option<Vec<InventoryEventsDetailed>> {
        self.storage
            .get_events(sku)
            .expect("Unable to retrieve events")
    }
}
