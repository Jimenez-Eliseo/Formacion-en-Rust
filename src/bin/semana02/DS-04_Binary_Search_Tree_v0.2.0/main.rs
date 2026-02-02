struct Node {
    valor: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // se crea un nodo con un valor
    // y sus hijo izquierdo y hijo derecho
    // esta en None porque  no estan asociados a nada
    fn new(valor: i32) -> Self {
        Node {
            valor: valor,
            left: None,
            right: None,
        }
    }
    // se usa para poder ligar o asociar al hijo izquierdo
    // dandole un nodo con quien se asocuara
    fn _set_left(&mut self, nodo: Box<Node>) {
        self.left = Some(nodo);
    }
    // se usa para poder ligar o asociar al hijo derecho
    // dandole un nodo con quien se asocuara
    fn _set_right(&mut self, nodo: Box<Node>) {
        self.right = Some(nodo);
    }
}

struct Arbol {
    raiz: Option<Box<Node>>,
}

impl Arbol {
    // creamos un arbol vacio
    // que no tiene una raiz
    // por eso creamos un arbol con una raiz None
    fn new() -> Self {
        Self { raiz: None }
    }

    // cuando tenemos que cambiar la raiz
    // con otro nodo que lo pasas como parametro
    fn _cambiar_raiz(&mut self, raiz: Node) {
        self.raiz = Some(Box::new(raiz));
    }

    // obtenemos el nodo raiz para uso general
    fn _obtener_raiz(&self) -> Option<&Box<Node>> {
        self.raiz.as_ref()
    }

    // usamos una funcion insertar que empezaremos
    // desde la raiz en caso de no tener nada en la raiz
    // creamos un nodo raiz para poder ahi asignar el valor
    // caso contrario se hace una insercion en profundidad haciendo
    // recursividad llamando a la funcion insertar_profundidad
    fn insertar(&mut self, numero: i32) {
        if self.raiz.is_none() {
            self.raiz = Some(Box::new(Node::new(numero)));
        } else {
            Self::insertar_profundidad(self.raiz.as_mut().unwrap(), numero);
        }
    }
    // se hace la insercion tendio el nodo verificando
    // si el valor del nodo actual es manor
    // si no se tiene un hijo izquierdo se hace ahi la insercion
    // si se tiene un hijo izquierdo se recorre ese nodo con la
    // llamada recursiva
    // lo mismo y propio a el hijo derecho siel valor es menor
    // a el numero que intentamos ionsertar si no har hijo derecho se hace
    // la creacion de un nodo con ese valor
    // y si tiene se hace recorre a ese nodo con la llamada recursiva
    fn insertar_profundidad(nodo: &mut Box<Node>, numero: i32) {
        if numero < nodo.valor {
            if nodo.left.is_none() {
                nodo.left = Some(Box::new(Node::new(numero)));
            } else {
                Self::insertar_profundidad(nodo.left.as_mut().unwrap(), numero);
            }
        } else if nodo.right.is_none() {
            nodo.right = Some(Box::new(Node::new(numero)));
        } else {
            Self::insertar_profundidad(nodo.right.as_mut().unwrap(), numero);
        }
    }

    fn buscar(&self, numero: i32) -> bool {
        Self::buscar_recursivo(&self.raiz, numero)
    }
    // hacemos la bsuqueda con un match porque
    // al tener como hijo derecho o izquierdo
    // es del tipo Option<Box<Node>>
    // este puede ser Option un enum que tiene
    // None cuando no hay nada signado
    // Some(Box<None>) si hay asignacion
    // y que para buscar igualamos el valor de cada nodo que visitamos
    // y si es menor se va para la izquierda y seguir buscando
    // y si es mayor se va para la derecha y seguir buscando
    // si en encontramos el valor devuelve "true"
    // caso contrario sera "false"
    // porque llego a el final de las hojas
    fn buscar_recursivo(nodo: &Option<Box<Node>>, numero: i32) -> bool {
        match nodo {
            None => false,
            Some(n) => {
                if n.valor == numero {
                    true
                } else if numero < n.valor {
                    Self::buscar_recursivo(&n.left, numero)
                } else {
                    Self::buscar_recursivo(&n.right, numero)
                }
            }
        }
    }
}

fn _preorden(nodo: &Option<Box<Node>>) {
    if let Some(n) = nodo {
        println!("{}", n.valor);
        _preorden(&n.left);
        _preorden(&n.right);
    }
}

fn _inorden(nodo: &Option<Box<Node>>) {
    if let Some(n) = nodo {
        _inorden(&n.left);
        println!("{}", n.valor);
        _inorden(&n.right);
    }
}

fn _postorden(nodo: &Option<Box<Node>>) {
    if let Some(n) = nodo {
        _postorden(&n.left);
        _postorden(&n.right);
        println!("{}", n.valor);
    }
}

#[cfg(test)]
mod tests {
    use super::Arbol;
    use super::Node;
    use std::time::Instant;

    // al tener la funcion de "mitades" tendremos el orden y validaremos estos encada nodo usando
    // el puntero que indica que valor deberia de estar en ese nodo y como el la funcion "mitades no
    // fuimos primero por la izquierda es en ese orden que se acumularon los valores leuog ala derecha
    // osea que seguiremos ese orden primero recusividad a la izquierda y luego la derecha"
    // el puntero que usaremos es compartido en todas las llamdas asi que es un puntero global
    // para toda la validacion
    fn validar(nodo: &Option<Box<Node>>, puntero: &mut usize, valores: &Vec<i32>) {
        if let Some(n) = nodo {
            let valor_esperado = valores[*puntero];
            assert_eq!(n.valor, valor_esperado, "valores diferentes");

            *puntero += 1;
            validar(&n.left, puntero, valores);
            validar(&n.right, puntero, valores);
        }
    }

    // usaremos el divide y venceras que tambien aplica el merge sort
    // le daremos unos indices "l" el limite alcanzable del lado izquierdo
    // "r" el limite del lado derecho
    // para que nos servira esto?
    // nos sirve para poder crear un arbol balanceado
    // y ademas lo guardamos en un acumulador parapoder tener el orden
    // para poder insertar y validar la estructura
    fn armando_arbol_balanceado(l: i32, r: i32, acumulador: &mut Vec<i32>) {
        if l == r {
            return;
        }
        let mitad = (l + r) / 2;
        acumulador.push(mitad);
        armando_arbol_balanceado(l, mitad, acumulador);
        armando_arbol_balanceado(mitad + 1, r, acumulador);
    }

    #[test]
    fn insercion_validacion() {
        let mut arbol = Arbol::new();
        let mut acumulador: Vec<i32> = Vec::new();
        armando_arbol_balanceado(0, 100_000, &mut acumulador);
        for i in &acumulador {
            arbol.insertar(*i);
        }
        let mut indice = 0;
        validar(&arbol.raiz, &mut indice, &acumulador);
    }

    #[test]
    fn buscar_is_correct() {
        let mut arbol = Arbol::new();
        let valores = vec![10, 5, 20, 2, 7, 15, 25];
        for valor in valores {
            arbol.insertar(valor);
        }
        //si encuentra el valor este respondera con true
        assert_eq!(arbol.buscar(7), true, "No se encontro el valor buscado");
        //si no se encuentra el valor devolvera false
        assert_eq!(arbol.buscar(6), false, "Si se encontro el valor buscado");
    }

    fn buqueda_lineal(vector_busqueda: &Vec<i32>, numero: i32) -> bool {
        for valor in vector_busqueda {
            if valor == &numero {
                return true;
            }
        }
        false
    }

    // al hacer la busqueda por el BST
    // tiene una complegidad de O(log(n))
    // por ser balanceado
    // hacer el vs con una vusqueda lineal
    // esto tiene complegidad de O(n)
    // que es menos eficiente en busqueda
    #[test]
    fn benchmark_bst_vs_buqueda_lineal() {
        let mut arbol = Arbol::new();
        let mut acumulador: Vec<i32> = Vec::new();

        armando_arbol_balanceado(0, 100_000, &mut acumulador);

        for i in &acumulador {
            arbol.insertar(*i);
        }
        let inicio_bst = Instant::now();

        assert_eq!(arbol.buscar(8989), true, "No se encontro el valor buscado");

        let fin_bst = inicio_bst.elapsed();

        acumulador.sort();
        let inicio_lineal_ordenado = Instant::now();

        assert_eq!(
            buqueda_lineal(&acumulador, 8989),
            true,
            "No se encontro el valor"
        );

        let fin_lineal_ordenado = inicio_lineal_ordenado.elapsed();

        println!("--- BENCHMARK ---");
        println!("Busqueda en BST de 100_000 items: {:?}", fin_bst);
        println!(
            "Busqueda lienal con lista ordenada: {:?}",
            fin_lineal_ordenado
        );
    }
}
