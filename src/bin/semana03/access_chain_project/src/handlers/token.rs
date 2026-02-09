use crate::{AccessHandler, AuthError, Request};

// Si es None, este handler es el último
pub struct TokenValidatorHandler {
    next: Option<Box<dyn AccessHandler>>,
}

impl TokenValidatorHandler {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl AccessHandler for TokenValidatorHandler {
    // Conecta este handler con el siguiente en la cadena
    fn set_next(&mut self, next: Box<dyn AccessHandler>) {
        self.next = Some(next);
    }

    // 1. Verifica si el token es válido
    // 2. Si no es válido → se corta la cadena y se retorna error
    // 3. Si es válido → se pasa la petición al siguiente handler
    //
    // estamos quemando datos con "validar_firma"
    fn handle(&self, request: &Request) -> Result<(), AuthError> {
        if request.token != "validar_firma" {
            return Err(AuthError::InvalidToken);
        }
        println!("TOKEN OK");
        if let Some(next) = &self.next {
            return next.handle(request);
        }
        Ok(())
    }
}
