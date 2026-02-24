mod config;

use config::ConfigError;
use config::load_database_config;

pub fn start_microservice(connect: &mut bool) -> Result<(), ConfigError> {
    // intentar cargar configuración
    let _config = load_database_config()?;

    // conexión a base de datos
    // apuntando con *
    *connect = true;
    Ok(())
}
