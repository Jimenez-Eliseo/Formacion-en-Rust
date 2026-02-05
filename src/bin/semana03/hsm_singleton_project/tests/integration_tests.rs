use hsm_singleton_project::HSMService;
use std::sync::Arc;
use std::thread;

#[test]
fn test_multithreads_access() {
    let hsm = Arc::new(HSMService::new());

    let mut handles = vec![];

    let num_hilos = 4;
    let operaciones_por_hilo = 25;
    for _ in 0..num_hilos {
        let hsm_clonado = hsm.clone();

        let handle = thread::spawn(move || {
            for _ in 0..operaciones_por_hilo {
                hsm_clonado
                    .sign_data("mensaje de prueba")
                    .expect("Error firmando datos");
            }
        });

        handles.push(handle);
    }
    // Esperar a que todos los hilos terminen
    for h in handles {
        h.join().unwrap();
    }

    // Verificar contador global
    let total = hsm
        .obtener_operation_count()
        .expect("Error leyendo contador");

    // 4 hilos * 25 operaciones = 100
    assert_eq!(total, num_hilos * operaciones_por_hilo);
}
