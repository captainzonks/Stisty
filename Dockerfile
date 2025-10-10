# ==============================================================================
# STISTY DOCKERFILE - Statistical Analysis WASM Frontend
# ==============================================================================
# Description: Multi-stage build for WASM-based genome analysis web interface
# Author: barhamm
# Created: 2025-10-07
# Modified: 2025-10-07
# Version: 1.0.0
# Host: Rome (Arch Linux / AMD Ryzen 5600x / Intel Arc A380)
# ==============================================================================
# Purpose: Build privacy-first genome analysis frontend with client-side processing
# Security: Non-root user, minimal Alpine image, Traefik + Authentik integration
# ==============================================================================

#=============================================================================
# BUILD STAGE - Compile Rust to WASM
#=============================================================================

FROM rust:1.90-alpine AS wasm-builder

WORKDIR /build

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    curl \
    bash

# Install wasm-pack and add wasm32 target
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rustup target add wasm32-unknown-unknown

# Copy source files
COPY . .

# Build WASM with proper target and getrandom config
ENV RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""
RUN cd stisty-wasm && sh build.sh

#=============================================================================
# RUNTIME STAGE - Nginx Static File Server
#=============================================================================

FROM nginx:alpine

# Create user and group (matching Rome conventions: UID:1000/GID:968)
ARG USER_ID=1000
ARG GROUP_ID=968
ARG USERNAME=stisty
ARG GROUPNAME=stisty

RUN addgroup -g ${GROUP_ID} -S ${GROUPNAME} \
    && adduser -u ${USER_ID} -D -S -G ${GROUPNAME} ${USERNAME}

# Copy nginx configuration
COPY stisty-wasm/nginx.conf /etc/nginx/conf.d/default.conf

# Copy built files from builder stage
COPY --from=wasm-builder /build/stisty-wasm/dist /usr/share/nginx/html

# Create health endpoint
RUN echo "OK" > /usr/share/nginx/html/health

# Create nginx cache directories and set permissions for non-root nginx
RUN mkdir -p /var/cache/nginx/client_temp \
             /var/cache/nginx/proxy_temp \
             /var/cache/nginx/fastcgi_temp \
             /var/cache/nginx/uwsgi_temp \
             /var/cache/nginx/scgi_temp \
    && chown -R ${USERNAME}:${GROUPNAME} /usr/share/nginx/html \
    && chown -R ${USERNAME}:${GROUPNAME} /var/cache/nginx \
    && chown -R ${USERNAME}:${GROUPNAME} /var/log/nginx \
    && chown -R ${USERNAME}:${GROUPNAME} /etc/nginx/conf.d \
    && touch /var/run/nginx.pid \
    && chown -R ${USERNAME}:${GROUPNAME} /var/run/nginx.pid \
    && chmod -R 755 /usr/share/nginx/html

# Switch to non-root user
USER ${USERNAME}

# Health check for container orchestration
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:8080/health || exit 1

# Expose port 8080 (Traefik will handle TLS and external access)
EXPOSE 8080

# Default environment variables
ENV NGINX_WORKER_PROCESSES=auto \
    NGINX_WORKER_CONNECTIONS=1024

# Run nginx in foreground
CMD ["nginx", "-g", "daemon off;"]
