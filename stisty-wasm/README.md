# Stisty Genome Analyzer - Web Interface

Privacy-first genome analysis running entirely in your browser using WebAssembly.

## Features

- 🔒 **100% Client-Side Processing** - Your genome data never leaves your device
- 🚀 **WebAssembly Performance** - Fast Rust code compiled to WASM
- 📊 **Interactive Analysis** - Real-time genome statistics and visualizations
- 🧬 **23andMe Compatible** - Supports 23andMe raw data format
- 🐳 **Easy Deployment** - Docker-based deployment with any reverse proxy

## Architecture

```
┌─────────────┐
│   Browser   │  ← All genome processing happens here
│   (WASM)    │
└──────┬──────┘
       │ HTTPS (static files only)
       ↓
┌─────────────┐
│   Reverse   │  ← Traefik, nginx, Caddy, etc.
│    Proxy    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   Docker    │  ← nginx serving static files
│  Container  │
└─────────────┘
```

## Quick Start

### Local Development

```bash
# Install dependencies
cargo install wasm-pack

# Build WASM
cd stisty-wasm
./build.sh

# Serve locally
cd dist
python3 -m http.server 8080

# Open browser
xdg-open http://localhost:8080
```

### Docker Deployment

```bash
# Build image
docker build -t stisty-genome-web -f stisty-wasm/Dockerfile .

# Run container
docker run -d \
  --name stisty-genome \
  -p 127.0.0.1:8080:80 \
  --restart unless-stopped \
  stisty-genome-web

# Or use docker-compose
cd stisty-wasm
docker-compose up -d
```

## Reverse Proxy Configuration

The container exposes port 80 internally. Configure your reverse proxy to handle SSL/TLS and external access.

### Traefik (Dynamic Configuration)

**File-based provider** (`dynamic/stisty.yml`):
```yaml
http:
  routers:
    stisty-genome:
      rule: "Host(`genome.yourdomain.com`)"
      entryPoints:
        - websecure
      service: stisty-genome
      tls:
        certResolver: letsencrypt
      middlewares:
        - authentik@file  # Optional: Add authentication

  services:
    stisty-genome:
      loadBalancer:
        servers:
          - url: "http://stisty-genome-web:80"
```

**Docker labels** (add to docker-compose.yml):
```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.stisty.rule=Host(`genome.yourdomain.com`)"
  - "traefik.http.routers.stisty.entrypoints=websecure"
  - "traefik.http.routers.stisty.tls.certresolver=letsencrypt"
  - "traefik.http.services.stisty.loadbalancer.server.port=80"
  # Optional: Authentik middleware
  - "traefik.http.routers.stisty.middlewares=authentik@file"
```

### Nginx

```nginx
server {
    listen 443 ssl http2;
    server_name genome.yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Caddy

```caddyfile
genome.yourdomain.com {
    reverse_proxy localhost:8080

    # Optional: Basic auth
    basicauth {
        username $2a$14$hashed_password
    }
}
```

## Authentication Integration

### Authentik (Recommended for Enterprise)

**Traefik Forward Auth Middleware**:

1. Create application in Authentik:
   - Type: Provider → Proxy Provider
   - Forward auth (single application)
   - External host: `https://genome.yourdomain.com`

2. Add Traefik middleware:
```yaml
# traefik/dynamic/middlewares.yml
http:
  middlewares:
    authentik:
      forwardAuth:
        address: http://authentik-server:9000/outpost.goauthentik.io/auth/traefik
        trustForwardHeader: true
        authResponseHeaders:
          - X-authentik-username
          - X-authentik-groups
          - X-authentik-email
          - X-authentik-name
```

3. Apply to Stisty router:
```yaml
http:
  routers:
    stisty-genome:
      middlewares:
        - authentik
```

### OAuth2 Proxy

```yaml
# Add oauth2-proxy service to docker-compose
oauth2-proxy:
  image: quay.io/oauth2-proxy/oauth2-proxy:latest
  command:
    - --provider=oidc
    - --client-id=YOUR_CLIENT_ID
    - --client-secret=YOUR_CLIENT_SECRET
    - --cookie-secret=RANDOM_SECRET
    - --email-domain=*
    - --upstream=http://stisty-genome-web:80
    - --http-address=0.0.0.0:4180
```

### Basic Auth (Simple)

```nginx
# In nginx config
location / {
    auth_basic "Genome Analyzer";
    auth_basic_user_file /etc/nginx/.htpasswd;
    proxy_pass http://127.0.0.1:8080;
}
```

```bash
# Generate password file
htpasswd -c /etc/nginx/.htpasswd username
```

## Security Considerations

### Data Privacy
- ✅ **All genome processing is client-side** - Data never uploaded to server
- ✅ **Static file serving only** - No backend processing or storage
- ✅ **No logs contain genetic data** - Only HTTP access logs
- ⚠️ **Use HTTPS** - Protect file upload in transit
- ⚠️ **Authentication recommended** - Limit access to trusted users

### Container Security
- Read-only root filesystem
- No new privileges
- Resource limits
- Health checks
- Minimal attack surface (static files only)

### Recommended Additional Measures
1. **Enable authentication** - Authentik, OAuth2 Proxy, or basic auth
2. **Use HTTPS only** - Let's Encrypt via reverse proxy
3. **IP whitelisting** - Restrict to known networks if possible
4. **Rate limiting** - Prevent abuse
5. **Security headers** - CSP, HSTS, X-Frame-Options (handled by reverse proxy)

## Environment Variables

The container accepts standard nginx environment variables:

```yaml
environment:
  - NGINX_WORKER_PROCESSES=auto
  - NGINX_WORKER_CONNECTIONS=1024
```

## Resource Requirements

**Minimum**:
- CPU: 0.1 cores
- Memory: 64 MB
- Disk: 20 MB

**Recommended**:
- CPU: 0.5 cores
- Memory: 256 MB
- Disk: 50 MB

## Monitoring

### Health Check
```bash
curl http://localhost:8080/health
# Expected: "OK"
```

### Logs
```bash
# Docker logs
docker logs stisty-genome-web

# Follow logs
docker logs -f stisty-genome-web
```

### Metrics (if using Prometheus)
```yaml
# Add to docker-compose.yml
labels:
  - "prometheus.io/scrape=true"
  - "prometheus.io/port=80"
  - "prometheus.io/path=/metrics"
```

## Troubleshooting

### WASM fails to load
- Check browser console for MIME type errors
- Ensure nginx serves `.wasm` files with `application/wasm`
- Verify CORS headers if serving from different domain

### File upload fails
- Check browser file size limits (usually 100MB+ is fine)
- Verify sufficient browser memory
- Try smaller genome file or different browser

### Authentication loop (Authentik)
- Verify external host matches exactly
- Check forward auth address is correct
- Ensure trust forward headers is enabled

### Container won't start
```bash
# Check logs
docker logs stisty-genome-web

# Verify health
docker inspect --format='{{.State.Health.Status}}' stisty-genome-web

# Test nginx config
docker exec stisty-genome-web nginx -t
```

## Development

### Project Structure
```
stisty-wasm/
├── Cargo.toml           # WASM crate configuration
├── src/
│   └── lib.rs          # Rust → WASM bindings
├── www/
│   ├── index.html      # Main HTML
│   ├── style.css       # Styles
│   └── app.js          # JavaScript application
├── build.sh            # Build script
├── Dockerfile          # Container image
├── docker-compose.yml  # Docker orchestration
├── nginx.conf          # Nginx configuration
└── README.md           # This file
```

### Building from Source
```bash
# Build WASM module
cd stisty-wasm
wasm-pack build --target web --out-dir pkg --release

# Manual build
./build.sh

# Build Docker image
docker build -t stisty-genome-web -f Dockerfile ..
```

### Testing
```bash
# Run Rust tests
cargo test

# Build and serve locally
./build.sh
cd dist && python3 -m http.server 8080
```

## Production Deployment Checklist

- [ ] HTTPS enabled via reverse proxy
- [ ] Authentication configured (Authentik/OAuth2/Basic)
- [ ] DNS configured for custom domain
- [ ] Firewall rules in place
- [ ] Container resource limits set
- [ ] Logging configured
- [ ] Health checks working
- [ ] Backup strategy for container image
- [ ] SSL certificate auto-renewal configured
- [ ] Security headers verified
- [ ] Rate limiting enabled (if public)

## License

MIT OR Apache-2.0 (same as parent project)

## Support

- **Documentation**: [Main README](../README.md)
- **Issues**: [GitHub Issues](https://github.com/captainzonks/Stisty/issues)
- **Genome Analysis Guide**: [GENOME_USAGE.md](../GENOME_USAGE.md)