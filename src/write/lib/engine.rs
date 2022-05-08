use anyhow::{Error, Result};

use super::{
    aggregate::Aggregate, commands::InventoryCommand, entity::ProductDetail,
    event_storage::EventStorage, notifier::Notifier,
};

pub struct Engine {
    memory_events: Box<dyn EventStorage>,
    notifier: Option<Box<dyn Notifier>>,
}

impl Engine {
    pub fn new(storage: Box<dyn EventStorage>, notifier: Option<Box<dyn Notifier>>) -> Self {
        Self {
            memory_events: storage,
            notifier,
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
            let event_id = new_event.get_id();
            self.memory_events
                .add_event(&sku, new_event)
                .expect("Unable to insert event");
            if let Some(notifier) = &self.notifier {
                notifier.notify_event(&sku, &event_id)
                    .unwrap_or_else(|_| println!("Failed to send notify"));
            }
        }
        Ok(())
    }

    fn get_product_or_new(&mut self, sku: &str) -> ProductDetail {
        match self.get_product(sku) {
            None => ProductDetail::new(sku.to_string()),
            Some(p) => p,
        }
    }

    pub fn get_product(&mut self, sku: &str) -> Option<ProductDetail> {
        let product_events = self
            .memory_events
            .get_events(sku)
            .expect("Unable to get events");
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
    use std::thread;

    use crossbeam_channel::{unbounded, Receiver, Sender};

    use crate::write::lib::{inmemory_storage::InMemory, notifier::MemoryNotifier};

    use super::*;

    #[test]
    fn execute_cmd_test() {
        let storage = InMemory::new();
        let mut engine = Engine::new(Box::new(storage), None);
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
        let mut engine = Engine::new(Box::new(storage), None);
        let product = engine.get_product("abc");
        assert!(product.is_none());
    }

    #[test]
    fn sell_unavailable_product() {
        let storage = InMemory::new();
        let mut engine = Engine::new(Box::new(storage), None);
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

    #[test]
    fn execute_cmd_with_notification_test() {
        let storage = InMemory::new();
        let (tx_memory, rx): (Sender<(String, String)>, Receiver<(String, String)>) = unbounded();

        let t2 = thread::spawn(move || {
            let mut notifications = Vec::new();
            while let Ok(message) = rx.recv() {
                notifications.push(message);
            }
            notifications
        });

        let memory_notification = MemoryNotifier { sender: tx_memory };

        let mut engine = Engine::new(Box::new(storage), Some(Box::new(memory_notification)));

        let cmd = InventoryCommand::AddProduct {
            sku: "abc".to_string(),
            qty: 5,
        };
        let result = engine.execute(cmd);
        assert!(result.is_ok());

        let cmd = InventoryCommand::SellProduct {
            sku: "abc".to_string(),
            qty: 2,
        };
        let result = engine.execute(cmd);
        if let Err(err) = result {
            println!("1 {}", err)
        }

        let cmd = InventoryCommand::SellProduct {
            sku: "abc".to_string(),
            qty: 1,
        };
        let result = engine.execute(cmd);
        if let Err(err) = result {
            println!("2 {}", err)
        }

        drop(engine);
        let notifications = t2.join().unwrap();
        assert_eq!(
            notifications,
            vec![
                ("abc".to_owned(), "product_added".to_owned()),
                ("abc".to_owned(), "product_sold".to_owned()),
                ("abc".to_owned(), "product_sold".to_owned()),
            ]
        );
    }
}
