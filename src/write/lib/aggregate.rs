use anyhow::Result;

use crate::events::InventoryEvents;

use super::commands::InventoryCommand;

pub trait Aggregate {
    fn aggregate_type() -> String;
    fn run_command(&self, cmd: InventoryCommand) -> Result<Vec<InventoryEvents>>;
    fn apply(&mut self, event: &InventoryEvents);
}
