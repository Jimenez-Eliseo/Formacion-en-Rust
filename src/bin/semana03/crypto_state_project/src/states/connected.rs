use crate::{ChannelState, SecureChannel};

pub struct Connected;

impl Connected {
    pub fn new() -> Self {
        Connected
    }
}

impl ChannelState for Connected {
    fn handle_payload(&self, channel: &SecureChannel, data: &str) -> Result<String, String> {
        // Verificamos que tenemos llave (deberíamos tenerla)
        match &channel.encryption_key {
            Some(key) => {
                // Simulamos cifrar los datos
                let encrypted = format!("[CIFRADO con {}]: {}", key, data);
                println!("Enviando datos cifrados: {}", data);
                Ok(encrypted)
            }
            None => Err("ERROR: No hay llave de cifrado disponible".to_string()),
        }
    }

    fn transition_next(self: Box<Self>) -> Box<dyn ChannelState> {
        Box::new(super::closed::Closed::new())
    }

    fn state_name(&self) -> &'static str {
        "Connected"
    }
}

