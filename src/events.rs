use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum InventoryEvents {
    ProductSold {
        sku: String,
        qty: i32,
        id: Option<i32>,
    },
    ProductAdded {
        sku: String,
        qty: i32,
        id: Option<i32>,
    },
}

impl InventoryEvents {
    pub fn get_id(&self) -> String {
        match self {
            Self::ProductSold { .. } => "product_sold".to_string(),
            Self::ProductAdded { .. } => "product_added".to_string(),
        }
    }
}
