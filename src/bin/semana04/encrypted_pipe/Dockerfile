# ETAPA 1: CONSTRUCCIÓN (builder)
# Usa una imagen con Rust para compilar
FROM rust:1.70 as builder

# Crea la carpeta /app dentro del contenedor
WORKDIR /app

# Copia los archivos de configuración de Rust
COPY Cargo.toml Cargo.lock ./

# Crea un main.rs temporal para compilar dependencias
# Esto acelera builds futuros (caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compila las dependencias (sin nuestro código aún)
RUN cargo build --release

# AHORA SÍ: copia nuestro código real
COPY src ./src

# Toca el archivo para forzar recompilación
RUN touch src/main.rs

# Compila nuestro código
RUN cargo build --release

# ETAPA 2: IMAGEN FINAL (más pequeña)
# Usa Debian slim (versión mínima)
FROM debian:bookworm-slim

# Instala certificados SSL y OpenSSL (necesarios para TLS)
RUN apt-get update && apt-get install -y \
    ca-certificates \    # Para verificar certificados SSL
    libssl-dev \         # Librerías de OpenSSL
    && rm -rf /var/lib/apt/lists/*  # Limpia caché para ahorrar espacio

WORKDIR /app

# Copia SOLO el binario compilado (no el código fuente)
COPY --from=builder /app/target/release/encrypted_pipe /app/encrypted_pipe

# Crea carpeta para certificados
RUN mkdir -p /app/certs

# Comando que se ejecuta al iniciar el contenedor
CMD ["/app/encrypted_pipe"]
