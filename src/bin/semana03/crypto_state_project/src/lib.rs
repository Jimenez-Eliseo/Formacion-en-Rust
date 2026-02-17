mod states;
use states::closed::Closed;
use states::connected::Connected;
use states::handshake::HandshakeInProgress;
use states::uninitialized::Uninitialized;

// Trait que define el comportamiento de TODOS los estados
pub trait ChannelState {
    // Maneja el envío de datos según el estado actual
    fn handle_payload(&self, channel: &SecureChannel, data: &str) -> Result<String, String>;

    // Cambia al siguiente estado
    fn transition_next(self: Box<Self>) -> Box<dyn ChannelState>;

    // Nombre del estado (útil para debugging)
    fn state_name(&self) -> &'static str;
}

// El Contexto: Nuestro canal seguro
pub struct SecureChannel {
    state: Box<dyn ChannelState>,
    encryption_key: Option<String>,
}

impl SecureChannel {
    // Crear un canal nuevo (comienza en Uninitialized)
    pub fn new() -> Self {
        SecureChannel {
            state: Box::new(Uninitialized::new()),
            encryption_key: None,
        }
    }

    // Iniciar el handshake
    pub fn start_handshake(&mut self) -> Result<String, String> {
        // Verificamos si podemos hacer handshake desde el estado actual
        if self.state.state_name() != "Uninitialized" {
            return Err(format!(
                "No se puede iniciar handshake desde estado {}",
                self.state.state_name()
            ));
        }

        println!("Iniciando handshake...");
        self.state = Box::new(HandshakeInProgress::new());
        Ok("Handshake iniciado".to_string())
    }

    // Completar el handshake (simulado)
    pub fn complete_handshake(&mut self) -> Result<String, String> {
        if self.state.state_name() != "HandshakeInProgress" {
            return Err("No hay handshake en progreso".to_string());
        }

        // Simulamos generar una llave
        self.encryption_key = Some("llave_secreta_123".to_string());
        println!("Handshake completado. Llave generada!");
        self.state = Box::new(Connected::new());
        Ok("Conexión establecida".to_string())
    }

    // Enviar datos (solo funciona en Connected)
    pub fn send_data(&self, data: &str) -> Result<String, String> {
        self.state.handle_payload(self, data)
    }

    // Cerrar el canal
    pub fn close(&mut self) -> Result<String, String> {
        if self.state.state_name() == "Closed" {
            return Err("El canal ya está cerrado".to_string());
        }

        println!("Cerrando canal...");
        self.encryption_key = None;
        self.state = Box::new(Closed::new());
        Ok("Canal cerrado".to_string())
    }

    // Para pruebas - obtener el estado actual
    pub fn current_state(&self) -> &'static str {
        self.state.state_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninitialized_no_puede_enviar() {
        let channel = SecureChannel::new();
        assert_eq!(channel.current_state(), "Uninitialized");

        let result = channel.send_data("hola");

        // Debe dar error
        assert!(result.is_err());
    }

    #[test]
    fn test_handshake_no_puede_enviar() {
        let mut channel = SecureChannel::new();
        channel.start_handshake().unwrap();
        assert_eq!(channel.current_state(), "HandshakeInProgress");

        let result = channel.send_data("hola");

        // Debe dar error
        assert!(result.is_err());
    }

    #[test]
    fn test_connected_puede_enviar() {
        let mut channel = SecureChannel::new();
        channel.start_handshake().unwrap();
        channel.complete_handshake().unwrap();
        assert_eq!(channel.current_state(), "Connected");

        let result = channel.send_data("mensaje secreto");

        // Debe funcionar
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_se_puede_iniciar_handshake_desde_connected() {
        let mut channel = SecureChannel::new();
        channel.start_handshake().unwrap();
        channel.complete_handshake().unwrap();

        // Intentar iniciar handshake desde Connected
        let result = channel.start_handshake();
        // Debe dar error
        assert!(result.is_err());
    }
}
