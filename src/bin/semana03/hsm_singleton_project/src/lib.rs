mod core;
mod errors;

use core::{HSMCore, Status};
use errors::HSMError;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, OnceLock};

static HSM_INSTANCE: OnceLock<Arc<Mutex<HSMCore>>> = OnceLock::new();

fn get_hsm_instance() -> Arc<Mutex<HSMCore>> {
    HSM_INSTANCE
        .get_or_init(|| Arc::new(Mutex::new(HSMCore::new())))
        .clone()
}

pub struct HSMService {
    core: Arc<Mutex<HSMCore>>,
}

impl HSMService {
    /// Internamente obtiene la instancia única del HSM (Singleton)
    /// usando `get_hsm_instance()`. Todas las instancias de `HSMService`
    /// comparten el mismo núcleo (`HSMCore`).
    pub fn new() -> Self {
        Self {
            core: get_hsm_instance(),
        }
    }

    /// Firma datos usando el HSM simulado.
    ///
    /// - Bloquea el acceso al HSM mediante un `Mutex`.
    /// - Marca el estado como `Busy`.
    /// - Incrementa el contador global de operaciones.
    /// - Genera un hash del contenido como firma simulada.
    /// - Restaura el estado a `Ready`.
    ///
    /// # Errores
    /// Retorna `HSMError::LockPoisoned` si no se puede obtener el lock del mutex.
    pub fn sign_data(&self, data: &str) -> Result<String, HSMError> {
        let mut hsm = match self.core.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(HSMError::LockPoisoned),
        };

        hsm.status = Status::Busy;

        // Incrementa el contador global de operaciones
        hsm.operation_count.fetch_add(1, Ordering::SeqCst);

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash_final = hasher.finish();

        // Restaura el estado a listo
        hsm.status = Status::Ready;

        Ok(format!("Firma generada: {:x}", hash_final))
    }

    /// Devuelve el número total de operaciones realizadas por el HSM.
    ///
    /// Accede de forma segura al estado interno y lee el contador global.
    ///
    /// # Errores
    /// Retorna `HSMError::LockPoisoned` si el mutex no puede ser bloqueado.
    pub fn obtener_operation_count(&self) -> Result<u32, HSMError> {
        match self.core.lock() {
            Ok(guard) => return Ok(guard.operation_count.load(Ordering::SeqCst)),
            Err(_) => return Err(HSMError::LockPoisoned),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation_counter() {
        let hsm = HSMService::new();

        let _ = hsm.sign_data("a");
        let _ = hsm.sign_data("b");

        let total = hsm.obtener_operation_count().unwrap();
        assert_eq!(total, 2);
    }
}
