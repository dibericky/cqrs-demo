use anyhow::{Error, Result};
use serde::Serialize;

use crate::{aggregate::Aggregate, commands::InventoryCommand, events::InventoryEvents};

#[derive(Debug, Serialize)]
pub struct ProductDetail {
    pub sku: String,
    pub qty: i32,
}

impl ProductDetail {
    pub fn new(sku: String) -> Self {
        Self { sku, qty: 0 }
    }
}

impl Aggregate for ProductDetail {
    fn run_command(&self, cmd: InventoryCommand) -> Result<Vec<InventoryEvents>> {
        match cmd {
            InventoryCommand::AddProduct { sku, qty } => {
                let events = vec![InventoryEvents::ProductAdded { sku, qty, id: None }];
                Ok(events)
            }
            InventoryCommand::SellProduct { sku, qty } => {
                if self.qty - qty < 0 {
                    return Err(Error::msg(format!(
                        "Not enough quantity for product {}",
                        sku
                    )));
                }
                let events = vec![InventoryEvents::ProductSold { sku, qty, id: None }];
                Ok(events)
            }
        }
    }

    fn apply(&mut self, event: &InventoryEvents) {
        match event {
            InventoryEvents::ProductAdded { qty, .. } => self.qty += qty,
            InventoryEvents::ProductSold { qty, .. } => self.qty -= qty,
        }
    }

    fn aggregate_type() -> String {
        "Inventory".to_string()
    }
}
