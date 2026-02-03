# Como probar los test de Merge Sort

Si todo el proyecto esta clonado se usara el siguiente comando:

```
cargo test --bin mergesort -- --nocapture
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

## Demostracion de la complegidad algoritmica de mergesort

El caso base que tenemos es 1, 2T espo que se llama 2 veces a la funciona recursiva, n/2 es porque dividimos el slic, mas el n de que se encarga el merge que en el peor caso es n.

Tenemos aqui ecuacion de recurrencia:

$$
T(n)= \begin{cases}1 & \text { si } n=1 \\ 2 \cdot T\left(\frac{n}{2}\right)+n & \text { en otros casos }\end{cases}
$$
Solucion:
$$
\begin{aligned}
T(n) & =2 T\left(\frac{n}{2}\right)+n \\
T\left(\frac{n}{2}\right) & =2 T\left(\frac{n}{2^{2}}\right)+\frac{n}{2} \\
T(n) & =2\left(2 T\left(\frac{n}{2^{2}}\right)+\frac{n}{2}\right)+n \\
T(n) & =2^{2} T\left(\frac{n}{2^{2}}\right)+2 \cdot \frac{n}{2}+n \\
T\left(\frac{n}{2^{2}}\right) & =2 T\left(\frac{n}{2^{3}}\right)+\frac{n}{2^{2}} \\
T(n) & =2^{2}\left(2 T\left(\frac{n}{2^{3}}\right)+\frac{n}{2^{2}}\right)+2 \cdot \frac{n}{2}+n \\
T(n) & =2^{3} T\left(\frac{n}{2^{3}}\right)+2^{2} \cdot \frac{n}{2^{2}}+2 \cdot \frac{n}{2}+n \\
T\left(\frac{n}{2^{3}}\right) & =2 T\left(\frac{n}{2^{4}}\right)+\frac{n}{2^{3}} \\
T(n) & =2^{3}\left(2 T\left(\frac{n}{2^{4}}\right)+\frac{n}{2^{3}}\right)+2^{2} \cdot \frac{n}{2^{2}}+2 \cdot \frac{n}{2}+n \\
T(n) & =2^{4} T\left(\frac{n}{2^{4}}\right)+\not 2^{3} \cdot \frac{n}{2^{3}}+\not 2^{2} \cdot \frac{n}{\not 2^{2}}+\not 2 \cdot \frac{n}{\not 2}+n \\
T(n) & =2^{4} T\left(\frac{n}{2^{4}}\right)+n+n+n+n \\
T(n) & =2^{4} T\left(\frac{n}{2^{4}}\right)+4 \cdot n \\
\vdots & \\
T(n) & =2^{k} T\left(\frac{n}{2^{k}}\right)+k \cdot n
\end{aligned}
$$

Cuando $T(1)=1$, entonces $\frac{n}{2^{k}}=1$

$$
\begin{aligned}
\frac{n}{2^{k}} & =1 \\
n & =2^{k} \\
\log _{2} n & =\log _{2} 2^{k} \\
\log _{2} n & =k \cdot \log _{2} 2 \\
\log _{2} n & =k
\end{aligned}
$$
Reemplazando en la ecuación anterior
$$
\begin{aligned}
T(n) & =2^{k} T\left(\frac{n}{2^{k}}\right)+k \cdot n \\
T(n) & =2^{\log _{2} n} T\left(\frac{n}{2^{\log _{2} n}}\right)+n \cdot \log _{2} n \\
T(n) & =n \cdot T\left(\frac{n}{n}\right)+n \cdot \log _{2} n \\
T(n) & =n+n \cdot \log _{2} n \\
O(T(n)) & =n \cdot \log _{2} n \\
O(T(n)) & =n \cdot \log n
\end{aligned}
$$
