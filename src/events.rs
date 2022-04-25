#[derive(Clone, Debug)]
pub enum InventoryEvents {
    ProductSold { sku: String, qty: i32 },
    ProductAdded { sku: String, qty: i32 },
}
