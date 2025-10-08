# Multi-stage build for optimal image size

# Stage 1: Build WASM
FROM rust:1.90-alpine AS wasm-builder

WORKDIR /build

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    curl \
    bash

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy source files
COPY . .

# Build WASM
RUN cd stisty-wasm && sh build.sh

# Stage 2: Nginx static file server
FROM nginx:alpine

# Copy basic nginx config
COPY stisty-wasm/nginx.conf /etc/nginx/conf.d/default.conf

# Copy built files from builder stage
COPY --from=wasm-builder /build/stisty-wasm/dist /usr/share/nginx/html

# Add healthcheck for container orchestration
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:80/health || exit 1

# Expose port 80 (reverse proxy will handle external access)
EXPOSE 80

# Run nginx in foreground
CMD ["nginx", "-g", "daemon off;"]