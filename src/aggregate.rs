use anyhow::Result;

use crate::{commands::InventoryCommand, events::InventoryEvents};

pub trait Aggregate {
    fn aggregate_type() -> String;
    fn run_command(&self, cmd: InventoryCommand) -> Result<Vec<InventoryEvents>>;
    fn apply(&mut self, event: &InventoryEvents);
}
