use crate::{ChannelState, SecureChannel};

pub struct HandshakeInProgress;

impl HandshakeInProgress {
    pub fn new() -> Self {
        HandshakeInProgress
    }
}

impl ChannelState for HandshakeInProgress {
    fn handle_payload(&self, _channel: &SecureChannel, data: &str) -> Result<String, String> {
        Err("ERROR: Handshake en progreso, no se pueden enviar datos aún".to_string())
    }

    fn transition_next(self: Box<Self>) -> Box<dyn ChannelState> {
        Box::new(super::connected::Connected::new())
    }

    fn state_name(&self) -> &'static str {
        "HandshakeInProgress"
    }
}
