use std::collections::HashMap;

use anyhow::Result;

use crate::{event_storage::EventStorage, events::InventoryEvents};

pub struct InMemory {
    events: HashMap<String, Vec<InventoryEvents>>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }
}

impl Default for InMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStorage for InMemory {
    fn add_event(&mut self, key: &str, event: InventoryEvents) -> Result<()> {
        if !self.exists(key).unwrap() {
            self.events.insert(key.to_string(), Vec::new());
        }
        self.events.get_mut(key).unwrap().push(event);
        Ok(())
    }

    fn exists(&mut self, key: &str) -> Result<bool> {
        Ok(self.events.contains_key(key))
    }

    fn get_events(&mut self, key: &str) -> Result<Option<Vec<InventoryEvents>>> {
        Ok(self.events.get(key).map(|v| v.to_owned()))
    }
}
