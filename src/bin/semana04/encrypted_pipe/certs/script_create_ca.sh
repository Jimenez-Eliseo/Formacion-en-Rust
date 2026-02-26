#!/bin/bash
# 1. CA
#
cd certs

# como autoridad
# ca.key -> firma secreta 
# ca.crt -> placa o certificado 
openssl req -new -x509 -days 30 -nodes \
    -out ca.crt -keyout ca.key \
    -subj "/CN=MyLocalCA"

# Generamos la llave privada del servidor Postgres.
openssl genrsa -out server.key 2048

# configuramos para permitir múltiples nombres (SAN)
cat > server.conf <<EOF
[req]
prompt = no
distinguished_name = dn
req_extensions = ext
[dn]
CN = postgres_ssl
[ext]
subjectAltName = DNS:postgres_ssl, DNS:localhost, IP:127.0.0.1
EOF

# 4. Solicitud de firma (CSR)
# como servidor
# server.key -> firma secreta 
# server.crt -> placa o certificado
# server.conf -> configuracion extra en este caso nombres para el network
openssl req -new -key server.key -out server.csr -config server.conf

# firmar con la CA
# el server.crt es el certificado final ya firmado
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key \
    -CAcreateserial -out server.crt -days 365 \
    -extfile server.conf -extensions ext

# chmod 600 server.key

# esto hacer despues por separado
# sudo chown 999:999 server.key
