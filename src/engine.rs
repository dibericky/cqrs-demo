use anyhow::{Error, Result};

use crate::{
    aggregate::Aggregate, commands::InventoryCommand, entity::ProductDetail,
    event_storage::EventStorage,
};

pub struct Engine {
    memory_events: Box<dyn EventStorage>,
}

impl Engine {
    pub fn new(storage: Box<dyn EventStorage>) -> Self {
        Self {
            memory_events: storage,
        }
    }

    pub fn execute(&mut self, cmd: InventoryCommand) -> Result<()> {
        let sku = cmd.get_sku();

        let mut product = self.get_product_or_new(&sku);

        let new_events = match product.run_command(cmd) {
            Err(msg) => {
                let err_msg = format!("Unable to run command: {}", msg);
                return Err(Error::msg(err_msg));
            }
            Ok(events) => events,
        };
        for event in &new_events {
            product.apply(event);
        }
        for new_event in new_events {
            self.memory_events.add_event(&sku, new_event);
        }
        Ok(())
    }

    fn get_product_or_new(&self, sku: &str) -> ProductDetail {
        match self.get_product(sku) {
            None => ProductDetail::new(sku.to_string()),
            Some(p) => p,
        }
    }
    pub fn get_product(&self, sku: &str) -> Option<ProductDetail> {
        let product_events = self.memory_events.get_events(sku);
        match product_events {
            None => None,
            Some(events) => {
                let product = ProductDetail::new(sku.to_owned());
                let final_product = events.iter().fold(product, |mut prod, event| {
                    prod.apply(event);
                    prod
                });
                Some(final_product)
            }
        }
    }
}

#[cfg(test)]
mod test_engine {
    use crate::inmemory_storage::InMemory;

    use super::*;

    #[test]
    fn execute_cmd_test() {
        let storage = InMemory::new();
        let mut engine = Engine::new(Box::new(storage));
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

        let cmd = InventoryCommand::SellProduct {
            sku: "abc".to_string(),
            qty: 2,
        };
        let result = engine.execute(cmd);
        assert!(result.is_ok());
        let product = engine.get_product("abc").unwrap();
        assert_eq!(product.sku, "abc");
        assert_eq!(product.qty, 6);
    }

    #[test]
    fn get_product_with_no_events_test() {
        let storage = InMemory::new();
        let engine = Engine::new(Box::new(storage));
        let product = engine.get_product("abc");
        assert!(product.is_none());
    }

    #[test]
    fn sell_unavailable_product() {
        let storage = InMemory::new();
        let mut engine = Engine::new(Box::new(storage));
        let cmd = InventoryCommand::AddProduct {
            sku: "abc".to_string(),
            qty: 3,
        };
        let _ = engine.execute(cmd);

        let product = engine.get_product("abc").unwrap();
        assert_eq!(product.sku, "abc");
        assert_eq!(product.qty, 3);

        let cmd = InventoryCommand::SellProduct {
            sku: "abc".to_string(),
            qty: 10,
        };
        let result = engine.execute(cmd);
        assert!(result.is_err());

        let product = engine.get_product("abc").unwrap();
        assert_eq!(product.sku, "abc");
        assert_eq!(product.qty, 3);
    }
}
