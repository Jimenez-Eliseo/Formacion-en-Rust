# Como probar los test de BST

Si todo el proycto esta clonado se usara el siguiente comando:

```
cargo test --bin bst -- --nocapture
```

> [!TIP]
> [[bin]]
>
> name = "nombre_que_quieres_poner_como_etiqueta"
>
> path = "RUTA/HACIA/TU/main.rs"

Y usar el comando siguiente:

```
cargo test --bin NOMBRE_DE_ETIQUETA -- --nocapture
```

## Arbol BST

Un arbol de busqueda binaria se caracteriza de tener solo 2 hijos, el derecho y el izquierdo con la caracteristica de que el hijo izquierdo es menor que el padre,
el hijo derecho es mayor que el padre.

No se permiten valores repetidos.

Al momento de insertar se va agrandando el arbol y siguiendo esa regla.

Ahora un arbol balanceado es un que cuando esta dentro de  estas posibilidades:

- -1
- 0
- 1

La diferenica la altura del lado izquierdo - lado derecho, empezando desde el nodo actual.

Y al ser balanceado nos garantizamos la complegidad de **O(n)**, al no ser balanceado en el peor caso se hace un complegidad lienal **O(n)**.

## Metodo de generar en un BST balanceado en un rango de 0 a N

Para para poder hacer la implementacion en de tener un arbol balanceadod se tiene las rotacines simples a la izquierda y a la derecha, como tambien rotaciones dobles a ambos lados tambien.

Para simplificar esto usamos lo que hace el algoritmo de **merge sort**, divide y venceras, tome unos punteros l=0 y r=n en sacamos la mitad(que sera el valor que tendra el nodo) y hacemos la llamada recursiva l, m(la mitad -> l + r / 2) esto para el hijo izquierdo y m + 1, r.

Asi tendresmos el arbol sementado en mitades para ambos lados y tener un arbol balanceado.


