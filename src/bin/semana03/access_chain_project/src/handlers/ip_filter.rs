use crate::{AccessHandler, AuthError, Request};

// Lista de IPs que no tienen permitido el acceso
// siguiente eslabón de la cadena
// si es None significa que este es el último handler
pub struct IpFilterHandler {
    pub denied_ips: Vec<String>,
    pub next: Option<Box<dyn AccessHandler>>,
}

impl IpFilterHandler {
    pub fn new(denied_ips: Vec<String>) -> Self {
        Self {
            denied_ips,
            next: None,
        }
    }
}

impl AccessHandler for IpFilterHandler {
    // Recibe una lista de IPs bloqueadas y deja el siguiente handler vacío.
    // El siguiente handler se conecta después con `set_next`.
    // Esto permite construir la cadena:
    // IP -> Token -> Role -> ...
    fn set_next(&mut self, next: Box<dyn AccessHandler>) {
        self.next = Some(next);
    }

    // 1. Verifica si la IP está en la lista negra
    // 2. Si está bloqueada → corta la cadena y retorna error
    // 3. Si no está bloqueada → pasa la petición al siguiente handler
    fn handle(&self, request: &Request) -> Result<(), AuthError> {
        if self.denied_ips.contains(&request.ip) {
            return Err(AuthError::InvalidIP);
        }
        println!("IP OK {}", request.ip);
        if let Some(next) = &self.next {
            return next.handle(request);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tetst {
    use super::*;

    #[test]
    fn test_blocks_blacklisted_ip() {
        let denied_ips = vec!["192.168.1.100".to_string()];
        let handler = IpFilterHandler::new(denied_ips);

        let request = Request {
            ip: "192.168.1.100".to_string(), // IP Bloqueada
            token: "cualquier_cosa".to_string(),
            role: "user".to_string(),
        };

        // Verificación Unitaria (sin cadena, solo el handler)
        assert_eq!(handler.handle(&request), Err(AuthError::InvalidIP));
    }
}
