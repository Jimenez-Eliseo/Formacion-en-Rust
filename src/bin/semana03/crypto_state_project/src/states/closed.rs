use crate::{ChannelState, SecureChannel};

pub struct Closed;

impl Closed {
    pub fn new() -> Self {
        Closed
    }
}

impl ChannelState for Closed {
    fn handle_payload(&self, _channel: &SecureChannel, data: &str) -> Result<String, String> {
        Err("ERROR: Canal cerrado, no se pueden enviar datos".to_string())
    }

    fn transition_next(self: Box<Self>) -> Box<dyn ChannelState> {
        // Un canal cerrado no puede transicionar a otro estado
        self
    }

    fn state_name(&self) -> &'static str {
        "Closed"
    }
}
