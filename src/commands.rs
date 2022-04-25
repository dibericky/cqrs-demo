pub enum InventoryCommand {
    SellProduct { sku: String, qty: i32 },
    AddProduct { sku: String, qty: i32 },
}

impl InventoryCommand {
    pub fn get_sku(&self) -> String {
        match self {
            Self::AddProduct { sku, .. } => sku.to_string(),
            Self::SellProduct { sku, .. } => sku.to_string(),
        }
    }
}
