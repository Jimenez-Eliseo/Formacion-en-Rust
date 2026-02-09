use crate::{AccessHandler, AuthError, Request};

// Siguiente eslabón de la cadena de validación
// Si es None, este handler es el último
pub struct RoleCheckHandler {
    next: Option<Box<dyn AccessHandler>>,
}

// Inicialmente no tiene siguiente handler conectado.
impl RoleCheckHandler {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl AccessHandler for RoleCheckHandler {
    // Conecta este handler con el siguiente en la cadena
    fn set_next(&mut self, next: Box<dyn AccessHandler>) {
        self.next = Some(next);
    }

    // 1. Verifica si el rol del usuario es "Admin"
    // 2. Si no lo es → se corta la cadena y se retorna error
    // 3. Si es válido → se pasa la petición al siguiente handler
    //
    // estamos quemando datos con "Admin"
    fn handle(&self, request: &Request) -> Result<(), AuthError> {
        if request.role != "Admin" {
            return Err(AuthError::InvalidRole);
        }
        println!("Role Ok");
        if let Some(next) = &self.next {
            return next.handle(request);
        }
        Ok(())
    }
}
