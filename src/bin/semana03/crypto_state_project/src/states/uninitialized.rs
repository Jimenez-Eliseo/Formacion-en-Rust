use crate::{ChannelState, SecureChannel};

pub struct Uninitialized;

impl Uninitialized {
    pub fn new() -> Self {
        Uninitialized
    }
}

impl ChannelState for Uninitialized {
    fn handle_payload(&self, _channel: &SecureChannel, data: &str) -> Result<String, String> {
        Err("ERROR: No se puede enviar datos sin completar el handshake".to_string())
    }

    fn transition_next(self: Box<Self>) -> Box<dyn ChannelState> {
        Box::new(super::handshake::HandshakeInProgress::new())
    }

    fn state_name(&self) -> &'static str {
        "Uninitialized"
    }
}

