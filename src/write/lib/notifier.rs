use anyhow::Result;
use crossbeam_channel::Sender;

pub struct RabbitMQNotifier {}

impl Notifier for RabbitMQNotifier {
    fn notify_event(&self, _sku: &str, _event_type_id: &str) -> Result<()> {
        todo!();
    }
}

pub struct MemoryNotifier {
    pub sender: Sender<(String, String)>,
}

impl Notifier for MemoryNotifier {
    fn notify_event(&self, sku: &str, event_type_id: &str) -> Result<()> {
        self.sender
            .send((sku.to_owned(), event_type_id.to_owned()))?;
        Ok(())
    }
}

pub trait Notifier {
    fn notify_event(&self, sku: &str, event_type_id: &str) -> Result<()>;
}
