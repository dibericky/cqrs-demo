use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum InventoryEvents {
    ProductSold { sku: String, qty: i32 },
    ProductAdded { sku: String, qty: i32 },
}

#[derive(Clone, Debug, Serialize)]
pub struct EventDetailed {
    pub id: i32,
    pub sku: String,
    pub qty: i32,
    pub event_type_id: String,
    pub description: String,
}

pub enum InventoryEventsDetailed {
    ProductSold(EventDetailed),
    ProductAdded(EventDetailed),
}

impl From<InventoryEventsDetailed> for EventDetailed {
    fn from(inventory_event_detail: InventoryEventsDetailed) -> Self {
        match inventory_event_detail {
            InventoryEventsDetailed::ProductAdded(detail) => detail,
            InventoryEventsDetailed::ProductSold(detail) => detail,
        }
    }
}

impl InventoryEvents {
    pub fn get_id(&self) -> String {
        match self {
            Self::ProductSold { .. } => "product_sold".to_string(),
            Self::ProductAdded { .. } => "product_added".to_string(),
        }
    }
}

impl From<InventoryEventsDetailed> for InventoryEvents {
    fn from(detailed: InventoryEventsDetailed) -> Self {
        match detailed {
            InventoryEventsDetailed::ProductAdded(EventDetailed { sku, qty, .. }) => {
                InventoryEvents::ProductAdded { sku, qty }
            }
            InventoryEventsDetailed::ProductSold(EventDetailed { sku, qty, .. }) => {
                InventoryEvents::ProductSold { sku, qty }
            }
        }
    }
}
