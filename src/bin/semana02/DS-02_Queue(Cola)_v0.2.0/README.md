## Como probar los test de este Queue

Si todo el proyecto esta clonado se usara el siguiente comando:

```
cargo test --bin queue -- --nocapture
```

Caso contrario tendras que de clarar un objetivo para poder compilar tu archivo ya andamos dividiendo las carpetas por tareas, puedes hacerlo de esta manera editando el archivo **Cargo.toml** de esta manera:

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

# Vec vs VecDeque

### Vec

Tenemos al vector con la siguiente estructura:

- puntero
- len
- capacidad

Cuando se tiene un Vec::new() se tiene un espacio de memoria que ira pidiendo reasigncion, cuando se tenemos **len == capacidad** se hace una reasignacion que tiene un costo de O(n) que seria n = a la capacidad anterior,y el push entrante se aria en tiempo de O(1) asi que la reasignacion tiene un costo.

Cuando tenemos un **Vec::with_capacity(size)** se tiene una capacidad fija pero cuando se tiene que sobrepasar la capacidad el costo de O(n) se toma porque al sobre pasar la capacidad rust hace la reasignacion automaticamente.

### VecDeque

Tenemos un buffer circular, porque se usa aritmetica modular porque usa punteros de la y donde al hacer push se hace en tiempo de O(1) tanto por ambos lados pero en la reasignacion se hace lo mismo un O(n) que se hace un pedido de memoria no siempre el doble de capacidad pero es menajada demanera mas eficiente.


