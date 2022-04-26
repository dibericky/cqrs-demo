use std::collections::HashMap;

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
    fn add_event(&mut self, key: &str, event: InventoryEvents) {
        if !self.exists(key) {
            self.events.insert(key.to_string(), Vec::new());
        }
        self.events.get_mut(key).unwrap().push(event)
    }

    fn exists(&self, key: &str) -> bool {
        self.events.contains_key(key)
    }

    fn get_events(&self, key: &str) -> Option<Vec<InventoryEvents>> {
        self.events.get(key).map(|v| v.to_owned())
    }
}
