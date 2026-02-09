use access_chain_project::handlers::ip_filter::IpFilterHandler;
use access_chain_project::handlers::role::RoleCheckHandler;
use access_chain_project::handlers::token::TokenValidatorHandler;
use access_chain_project::{AccessHandler, AuthError, Request};

#[test]
fn test_tokem_expired() {
    let denied_ip = vec!["192.168.1.100".to_string()];

    let mut ip_handler = IpFilterHandler::new(denied_ip);
    let mut toke_handler = TokenValidatorHandler::new();
    let role_handler = RoleCheckHandler::new();

    // creamos el camino asi
    // a toke_handler se le set_next con role_handler
    // luego ip_handler se le set_next con toke_handler
    //
    // asi sera la construccion
    // y para las el uso de este se aplica primero
    // la ip -> token -> role
    //
    // Con Box::new() le damos la un objeto que haya implementado
    // el trait AccessHandler y de ese objeto le damos el
    // owership y este vivira mitras el handler ip_handler muera

    toke_handler.set_next(Box::new(role_handler));
    ip_handler.set_next(Box::new(toke_handler));

    let request = Request {
        ip: "192.168.1.10".to_string(),
        token: "validar_firmar".to_string(),
        role: "Admin".to_string(),
    };

    let result = ip_handler.handle(&request);

    // para la validacion de que se reciva el invalid token
    // se hace uso de el atributo PartialEq al enum AuthError
    // para poder hacer la validacion a nivel de boolean
    assert_eq!(result, Err(AuthError::InvalidToken));
}

#[test]
fn test_way_complet() {
    let denied_ip = vec!["192.168.1.100".to_string()];

    let mut ip_handler = IpFilterHandler::new(denied_ip);
    let mut toke_handler = TokenValidatorHandler::new();
    let role_handler = RoleCheckHandler::new();

    toke_handler.set_next(Box::new(role_handler));
    ip_handler.set_next(Box::new(toke_handler));

    let request = Request {
        ip: "192.168.1.10".to_string(),
        token: "validar_firma".to_string(),
        role: "Admin".to_string(),
    };

    let result = ip_handler.handle(&request);

    assert_eq!(result, Ok(()));
}
