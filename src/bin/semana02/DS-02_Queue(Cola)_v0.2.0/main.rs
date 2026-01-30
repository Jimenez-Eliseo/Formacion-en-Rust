use std::collections::VecDeque;

#[derive(Debug)]
struct MiCola<T> {
    items: VecDeque<T>,
}

impl<T> MiCola<T> {
    /// Crea una nueva instancia de `MiCola`.
    /// No asigna memoria hasta que se inserta el primer elemento.
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Agrega un elemento al final de la cola.
    /// Es una operación de tiempo constante O(1).
    fn push_back(&mut self, valor: T) {
        self.items.push_back(valor);
    }

    /// Agrega un elemento al inicio de la cola.
    /// A diferencia de un `Vec`, esto es O(1) y no desplaza los elementos.
    fn push_front(&mut self, valor: T) {
        self.items.push_front(valor);
    }

    /// Extrae y devuelve el último elemento de la cola.
    /// Retorna `Some(T)` si hay datos o `None` si la cola está vacía.
    fn pop_back(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    /// Extrae y devuelve el primer elemento (comportamiento FIFO).
    /// Si la cola está vacía devuelve `None`, de lo contrario devuelve `Some(valor)`.
    fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Verifica si la cola no tiene elementos.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Devuelve la cantidad exacta de elementos en la cola.
    fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::MiCola;
    use std::time::Instant;

    const LISTA_BASE: [i32; 4] = [65, 53, 2, 5];

    #[test]
    fn push_back_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        for &valor in LISTA_BASE.iter() {
            cola.push_back(valor);
        }

        assert_eq!(cola.items.len(), 4);
    }

    #[test]
    fn push_front_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        for &valor in LISTA_BASE.iter() {
            cola.push_front(valor);
        }

        assert_eq!(cola.items.len(), 4);
    }

    #[test]
    fn pop_back_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        for &valor in LISTA_BASE.iter() {
            cola.push_back(valor);
        }

        for &esperado in LISTA_BASE.iter().rev() {
            assert_eq!(cola.pop_back(), Some(esperado));
        }

        assert_eq!(cola.pop_back(), None);
    }

    #[test]
    fn pop_front_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        for &valor in LISTA_BASE.iter() {
            cola.push_back(valor);
        }

        for &esperado in LISTA_BASE.iter() {
            assert_eq!(cola.pop_front(), Some(esperado));
        }

        assert_eq!(cola.pop_back(), None);
    }

    #[test]
    fn is_empty_is_correct() {
        let cola: MiCola<i32> = MiCola::new();

        assert_eq!(cola.is_empty(), true);
    }

    #[test]
    fn len_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();

        assert_eq!(cola.len(), 0);

        for &valor in LISTA_BASE.iter() {
            cola.push_back(valor);
        }

        assert_eq!(cola.len(), 4);
    }

    #[test]
    fn insercion_back_y_extraccion_back_de_100000_elementos_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        let inicio = Instant::now();
        for i in 1..=100_000 {
            cola.push_back(i);
        }
        let duracion_insercion = inicio.elapsed();

        let mut contador_valido: i32 = 100_000;
        while let Some(elemento) = cola.pop_back() {
            assert_eq!(contador_valido, elemento, "Error: N sigue le orden FIFO");
            contador_valido -= 1;
        }
        let duracion_extraccion = inicio.elapsed();

        println!("---------------------------------------");
        println!("Duracion de insercion push_back: {:?}", duracion_insercion);
        println!("Duracion de extraccion pop_back: {:?}", duracion_extraccion);

        assert_eq!(cola.len(), 0, "Error: La cola deberia estar vacia");
        println!("---------------------------------------");
    }

    #[test]
    fn insercion_front_y_extraccion_front_de_100000_elementos_is_correct() {
        let mut cola: MiCola<i32> = MiCola::new();
        let inicio = Instant::now();
        for i in 1..=100_000 {
            cola.push_front(i);
        }
        let duracion_insercion = inicio.elapsed();

        let mut contador_valido: i32 = 100_000;
        while let Some(elemento) = cola.pop_front() {
            assert_eq!(contador_valido, elemento, "Error: N sigue le orden FIFO");
            contador_valido -= 1;
        }
        let duracion_extraccion = inicio.elapsed();

        println!("---------------------------------------");
        println!("Duracion de insercion push_front: {:?}", duracion_insercion);
        println!(
            "Duracion de extraccion pop_front: {:?}",
            duracion_extraccion
        );

        assert_eq!(cola.len(), 0, "Error: La cola deberia estar vacia");
        println!("---------------------------------------");
    }
}
