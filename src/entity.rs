use anyhow::Result;

use crate::{aggregate::Aggregate, commands::InventoryCommand, events::InventoryEvents};

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
                let events = vec![InventoryEvents::ProductAdded { sku, qty }];
                Ok(events)
            }
            InventoryCommand::SellProduct { sku, qty } => {
                let events = vec![InventoryEvents::ProductSold { sku, qty }];
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
