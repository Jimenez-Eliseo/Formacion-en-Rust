#[cfg(test)]
mod test {

    use std::time::Instant;

    // el costo de tener qu dividir recurivamente en 2
    // es que seria haciendo de manera decursiva hacia abajo
    // formando un arbol que tendra costo de
    // log(n) por cada llamada multiplicada junto a el merge
    // que es la funcion que hace la mezcla que en el peor
    // de los casos ira a recorrer todo el slice que seria n
    // osea que la complegidad seria
    // O(n * log n)
    // mas explicacion de la compleidad tambien estara
    // en el README.md de esta tarea
    fn merge_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
        if slice.len() <= 1 {
            return slice.to_vec();
        }

        let mid = slice.len() / 2;
        let (izq, der) = slice.split_at(mid);

        let izq_ord = merge_sort(izq);
        let der_ord = merge_sort(der);

        merge(&izq_ord, &der_ord)
    }

    fn merge<T: Ord + Clone>(izq: &[T], der: &[T]) -> Vec<T> {
        let mut resultado = Vec::with_capacity(izq.len() + der.len());

        let mut l = 0;
        let mut r = 0;

        while l < izq.len() && r < der.len() {
            if izq[l] <= der[r] {
                resultado.push(izq[l].clone());
                l += 1;
            } else {
                resultado.push(der[r].clone());
                r += 1;
            }
        }

        while l < izq.len() {
            resultado.push(izq[l].clone());
            l += 1;
        }

        while r < der.len() {
            resultado.push(der[r].clone());
            r += 1;
        }

        resultado
    }

    // el impacto en memoria que tendremos es que este
    // al tenre la funcion merge esta hace recepcion
    // de 2 slice uno derecho y el izquierdo
    // y retorna en un vextor el merge que
    // compara cada slice derecho e izquierdo
    // teniendo la clonacion de todo el slice
    //
    // asi que la clonacion por niveles es de n elementos
    // el numero de niveles es log_2(n)
    // el total de clonaciones n * log_2(n)
    //
    // a esto le multiplicamos la cuanto ocupa
    // un T en este aso i32
    #[test]
    fn validacion_de_impacto_de_memoria_y_tiempo_ordenacion() {
        let data: Vec<i32> = (0..100_000).rev().collect();

        let inicio = Instant::now();
        let _ = merge_sort(&data);
        let fin = inicio.elapsed();

        println!("Tiempo: {:?}", fin);
        println!(
            "Tamaño original: {} bytes",
            data.len() * std::mem::size_of::<i32>()
        );
    }
}
