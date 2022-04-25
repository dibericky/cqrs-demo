use std::collections::HashMap;

use anyhow::Result;

use crate::{
    aggregate::Aggregate, commands::InventoryCommand, entity::ProductDetail,
    events::InventoryEvents,
};

pub struct Engine {
    memory_events: HashMap<String, Vec<InventoryEvents>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            memory_events: HashMap::new(),
        }
    }

    fn initialize_memory_events_if_missing(&mut self, sku: &str) {
        if !self.memory_events.contains_key(sku) {
            self.memory_events.insert(sku.to_string(), Vec::new());
        }
    }
    pub fn execute(&mut self, cmd: InventoryCommand) -> Result<()> {
        let sku = cmd.get_sku();
        self.initialize_memory_events_if_missing(&sku);

        let mut product = self.get_product_or_new(&sku);

        let new_events = match product.run_command(cmd) {
            Err(_) => return Err(anyhow::Error::msg("Unable to run command")),
            Ok(events) => events,
        };
        for event in &new_events {
            product.apply(event);
        }
        let already_existing_events = self.memory_events.get_mut(&sku).unwrap();
        already_existing_events.extend(new_events.clone());
        Ok(())
    }

    fn get_product_or_new(&self, sku: &str) -> ProductDetail {
        match self.get_product(sku) {
            None => ProductDetail::new(sku.to_string()),
            Some(p) => p,
        }
    }
    pub fn get_product(&self, sku: &str) -> Option<ProductDetail> {
        let product_events = self.memory_events.get(sku);
        let product = ProductDetail::new(sku.to_owned());
        match product_events {
            None => None,
            Some(events) => {
                let final_product = events.iter().fold(product, |mut prod, event| {
                    prod.apply(event);
                    prod
                });
                Some(final_product)
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_engine {
    use super::*;

    #[test]
    fn execute_cmd_test() {
        let mut engine = Engine::new();
        let cmd = InventoryCommand::AddProduct {
            sku: "abc".to_string(),
            qty: 3,
        };
        let result = engine.execute(cmd);
        assert!(result.is_ok());

        let product = engine.get_product("abc");
        assert!(product.is_some());
        let product = product.unwrap();
        assert_eq!(product.sku, "abc");
        assert_eq!(product.qty, 3);

        let cmd = InventoryCommand::AddProduct {
            sku: "abc".to_string(),
            qty: 5,
        };
        let result = engine.execute(cmd);
        assert!(result.is_ok());
        let product = engine.get_product("abc").unwrap();
        assert_eq!(product.sku, "abc");
        assert_eq!(product.qty, 8);
    }

    #[test]
    fn get_product_with_no_events_test() {
        let engine = Engine::new();
        let product = engine.get_product("abc");
        assert!(product.is_none());
    }
}
