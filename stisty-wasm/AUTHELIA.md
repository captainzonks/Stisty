# Authelia Integration Guide

This guide covers integrating Stisty Genome Analyzer with Authelia for enterprise authentication and access control.

## Why Authelia?

- 🔐 **Forward Authentication** - Works seamlessly with Traefik, nginx, Caddy
- 👥 **LDAP/File Backend** - Flexible user management options
- 🔑 **MFA Support** - TOTP (Authy, Google Authenticator), WebAuthn, Duo
- 📊 **Access Control** - Fine-grained rules based on user, group, network
- 🚀 **Lightweight** - Single binary, minimal resource usage
- 🎯 **OpenID Connect** - OIDC provider capabilities

## Architecture

```
User Browser
     ↓
Traefik (Reverse Proxy)
     ↓
Authelia (Forward Auth Middleware)
     ↓ (authorized)
Stisty Container (WASM App)
```

## Prerequisites

- Traefik reverse proxy running
- Docker and docker-compose
- Domain name with DNS configured
- Redis (for session storage)
- Optional: LDAP server or use file-based users

## Setup Steps

### 1. Create Directory Structure

```bash
mkdir -p authelia/{config,users}
cd authelia
```

### 2. Create Authelia Configuration

**authelia/config/configuration.yml**:

```yaml
---
# Authelia Configuration for Stisty Genome Analyzer

server:
  host: 0.0.0.0
  port: 9091

log:
  level: info

theme: auto

# JWT secret for session tokens
jwt_secret: YOUR_RANDOM_JWT_SECRET_HERE_MIN_32_CHARS

# Default redirection URL (your domain)
default_redirection_url: https://genome.yourdomain.com

totp:
  issuer: genome.yourdomain.com
  period: 30
  skew: 1

authentication_backend:
  # Disable password reset (can enable with SMTP)
  password_reset:
    disable: false

  refresh_interval: 5m

  # File-based user database (alternative: LDAP)
  file:
    path: /config/users_database.yml
    password:
      algorithm: argon2id
      iterations: 1
      salt_length: 16
      parallelism: 8
      memory: 64

# Access Control Rules
access_control:
  default_policy: deny

  rules:
    # Authelia portal - bypass for login
    - domain: auth.yourdomain.com
      policy: bypass

    # Genome analyzer - require authentication + MFA
    - domain: genome.yourdomain.com
      policy: two_factor
      # Optional: Restrict to specific group
      subject:
        - "group:genome-users"

    # Optional: Allow specific networks without MFA
    - domain: genome.yourdomain.com
      policy: one_factor
      networks:
        - 192.168.1.0/24
      subject:
        - "group:genome-users"

# Session configuration
session:
  name: authelia_session
  domain: yourdomain.com  # Root domain for cookies
  same_site: lax
  secret: YOUR_RANDOM_SESSION_SECRET_HERE_MIN_32_CHARS
  expiration: 1h
  inactivity: 30m
  remember_me_duration: 1M

  redis:
    host: redis
    port: 6379
    # Optional: password: redis_password
    database_index: 0

# Storage for user preferences and device registration
storage:
  encryption_key: YOUR_RANDOM_ENCRYPTION_KEY_MIN_32_CHARS

  local:
    path: /config/db.sqlite3

# Notification provider (for password reset and device registration)
notifier:
  # Filesystem - for development/testing
  filesystem:
    filename: /config/notification.txt

  # SMTP - for production (uncomment and configure)
  # smtp:
  #   username: noreply@yourdomain.com
  #   password: smtp_password
  #   host: smtp.gmail.com
  #   port: 587
  #   sender: "Genome Analyzer <noreply@yourdomain.com>"
  #   subject: "[Genome Analyzer] {title}"
```

### 3. Create Users Database

**authelia/config/users_database.yml**:

```yaml
---
# User Database for Authelia
# Generate password hash with: docker run --rm authelia/authelia:latest authelia crypto hash generate argon2 --password 'yourpassword'

users:
  # Admin user
  admin:
    displayname: "Admin User"
    password: "$argon2id$v=19$m=65536,t=1,p=8$YOUR_HASHED_PASSWORD_HERE"
    email: admin@yourdomain.com
    groups:
      - admins
      - genome-users

  # Example genome researcher
  researcher1:
    displayname: "Jane Researcher"
    password: "$argon2id$v=19$m=65536,t=1,p=8$YOUR_HASHED_PASSWORD_HERE"
    email: jane@yourdomain.com
    groups:
      - genome-users

  # Example limited user
  analyst1:
    displayname: "John Analyst"
    password: "$argon2id$v=19$m=65536,t=1,p=8$YOUR_HASHED_PASSWORD_HERE"
    email: john@yourdomain.com
    groups:
      - genome-users
```

**Generate password hashes**:

```bash
# Generate hash for each user
docker run --rm authelia/authelia:latest authelia crypto hash generate argon2 --password 'your_secure_password'

# Example output:
# Digest: $argon2id$v=19$m=65536,t=1,p=8$abc123...
```

### 4. Generate Secrets

```bash
# Generate JWT secret
docker run --rm authelia/authelia:latest authelia crypto rand --length 64 --charset alphanumeric

# Generate session secret
docker run --rm authelia/authelia:latest authelia crypto rand --length 64 --charset alphanumeric

# Generate encryption key
docker run --rm authelia/authelia:latest authelia crypto rand --length 64 --charset alphanumeric
```

Update `configuration.yml` with generated secrets.

### 5. Configure Traefik Middleware

**traefik/dynamic/middlewares.yml**:

```yaml
http:
  middlewares:
    authelia:
      forwardAuth:
        address: http://authelia:9091/api/verify?rd=https://auth.yourdomain.com/
        trustForwardHeader: true
        authResponseHeaders:
          - Remote-User
          - Remote-Groups
          - Remote-Name
          - Remote-Email
```

**Or using Docker labels** (in Traefik's config):

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.middlewares.authelia.forwardauth.address=http://authelia:9091/api/verify?rd=https://auth.yourdomain.com/"
  - "traefik.http.middlewares.authelia.forwardauth.trustForwardHeader=true"
  - "traefik.http.middlewares.authelia.forwardauth.authResponseHeaders=Remote-User,Remote-Groups,Remote-Name,Remote-Email"
```

### 6. Docker Compose Configuration

**authelia/docker-compose.yml**:

```yaml
version: '3.8'

services:
  authelia:
    image: authelia/authelia:latest
    container_name: authelia
    restart: unless-stopped

    volumes:
      - ./config:/config

    networks:
      - traefik_net

    environment:
      - TZ=America/New_York

    labels:
      # Traefik routing for Authelia portal
      - "traefik.enable=true"
      - "traefik.http.routers.authelia.rule=Host(`auth.yourdomain.com`)"
      - "traefik.http.routers.authelia.entrypoints=websecure"
      - "traefik.http.routers.authelia.tls.certresolver=letsencrypt"
      - "traefik.http.services.authelia.loadbalancer.server.port=9091"

    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:9091/api/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 10s

  redis:
    image: redis:alpine
    container_name: authelia-redis
    restart: unless-stopped

    volumes:
      - redis-data:/data

    networks:
      - traefik_net

    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 3s
      retries: 3

volumes:
  redis-data:

networks:
  traefik_net:
    external: true
```

### 7. Update Stisty Service

**Update stisty-wasm/docker-compose.yml** to use Authelia middleware:

```yaml
version: '3.8'

services:
  stisty-genome-web:
    build:
      context: ..
      dockerfile: stisty-wasm/Dockerfile
    image: stisty-genome-web:latest
    container_name: stisty-genome-web
    restart: unless-stopped

    networks:
      - traefik_net

    labels:
      # Traefik routing
      - "traefik.enable=true"
      - "traefik.http.routers.stisty.rule=Host(`genome.yourdomain.com`)"
      - "traefik.http.routers.stisty.entrypoints=websecure"
      - "traefik.http.routers.stisty.tls.certresolver=letsencrypt"

      # Apply Authelia middleware
      - "traefik.http.routers.stisty.middlewares=authelia@file"

      # Service configuration
      - "traefik.http.services.stisty.loadbalancer.server.port=80"

networks:
  traefik_net:
    external: true
```

### 8. Deploy and Test

```bash
# Start Authelia and Redis
cd authelia
docker-compose up -d

# Check logs
docker logs authelia
docker logs authelia-redis

# Verify health
docker ps | grep authelia

# Start Stisty service
cd ../stisty-wasm
docker-compose up -d
```

**Test authentication flow**:

1. Navigate to `https://genome.yourdomain.com`
2. Should redirect to `https://auth.yourdomain.com`
3. Login with credentials
4. Complete MFA setup (TOTP code)
5. Redirect back to Stisty application

## Advanced Configuration

### Multi-Factor Authentication

#### TOTP (Authy, Google Authenticator, etc.)

Already configured in `configuration.yml`:

```yaml
totp:
  issuer: genome.yourdomain.com
  period: 30
  skew: 1  # Allow 1 period before/after for clock drift
```

**Setup flow**:
1. User logs in for first time
2. Authelia displays QR code
3. Scan with Authy or any TOTP app
4. Enter 6-digit code to verify
5. Device registered for future logins

#### WebAuthn (Hardware Keys)

Add to `configuration.yml`:

```yaml
webauthn:
  disable: false
  display_name: Genome Analyzer
  attestation_conveyance_preference: indirect
  user_verification: preferred
  timeout: 60s
```

#### Duo Push

```yaml
duo_api:
  hostname: api-XXXXXXXX.duosecurity.com
  integration_key: YOUR_INTEGRATION_KEY
  secret_key: YOUR_SECRET_KEY
  enable_self_enrollment: false
```

### Fine-Grained Access Control

**Group-based access**:

```yaml
access_control:
  rules:
    # Admins get full access
    - domain: genome.yourdomain.com
      policy: two_factor
      subject:
        - "group:admins"

    # Researchers limited to specific paths
    - domain: genome.yourdomain.com
      policy: two_factor
      resources:
        - "^/analysis.*$"
      subject:
        - "group:researchers"

    # Read-only analysts
    - domain: genome.yourdomain.com
      policy: one_factor
      methods:
        - GET
      subject:
        - "group:analysts"
```

**Network-based rules**:

```yaml
access_control:
  rules:
    # Internal network - less strict
    - domain: genome.yourdomain.com
      policy: one_factor
      networks:
        - 192.168.1.0/24
        - 10.0.0.0/8

    # External access - require MFA
    - domain: genome.yourdomain.com
      policy: two_factor
```

**Time-based access**:

Combined with external tools or custom rules:

```yaml
# Example: Only allow access during business hours
# (Requires custom Traefik plugin or network firewall rules)
```

### LDAP Integration

Replace file-based authentication with LDAP:

```yaml
authentication_backend:
  ldap:
    implementation: custom
    url: ldap://ldap.yourdomain.com
    timeout: 5s
    start_tls: false
    tls:
      skip_verify: false
      minimum_version: TLS1.2

    base_dn: dc=yourdomain,dc=com
    username_attribute: uid
    additional_users_dn: ou=users
    users_filter: (&({username_attribute}={input})(objectClass=person))

    additional_groups_dn: ou=groups
    groups_filter: (&(member={dn})(objectClass=groupOfNames))
    group_name_attribute: cn
    mail_attribute: mail
    display_name_attribute: displayName

    user: cn=admin,dc=yourdomain,dc=com
    password: ldap_password
```

### Session Management

**Adjust session timeouts**:

```yaml
session:
  expiration: 4h        # Total session duration
  inactivity: 1h        # Logout after inactivity
  remember_me_duration: 7d  # "Remember me" checkbox
```

**Per-domain sessions**:

```yaml
session:
  domain: yourdomain.com  # Shared across subdomains
  # OR
  cookies:
    - domain: genome.yourdomain.com
      authelia_url: https://auth.yourdomain.com
```

### Email Notifications

**Configure SMTP for production**:

```yaml
notifier:
  smtp:
    username: noreply@yourdomain.com
    password: your_smtp_password
    host: smtp.gmail.com
    port: 587
    sender: "Genome Analyzer <noreply@yourdomain.com>"
    identifier: genome.yourdomain.com
    subject: "[Genome Analyzer] {title}"
    startup_check_address: admin@yourdomain.com
    disable_require_tls: false
    disable_html_emails: false

    tls:
      skip_verify: false
      minimum_version: TLS1.2
```

**Template customization**:

```bash
# Override email templates
mkdir -p authelia/config/templates
# Place custom HTML templates in this directory
```

## Monitoring and Logs

### View Authelia Logs

```bash
# Real-time logs
docker logs -f authelia

# Filter for errors
docker logs authelia 2>&1 | grep -i error

# Authentication attempts
docker logs authelia 2>&1 | grep "authentication"
```

### Metrics (Prometheus)

Authelia exposes metrics at `/api/metrics`:

```yaml
# In Authelia config
telemetry:
  metrics:
    enabled: true
    address: tcp://0.0.0.0:9959

# Prometheus scrape config
scrape_configs:
  - job_name: 'authelia'
    static_configs:
      - targets: ['authelia:9959']
```

### Failed Login Monitoring

```bash
# Watch for failed attempts
docker logs -f authelia | grep -i "failed\|unauthorized"

# Count failed logins by user
docker logs authelia 2>&1 | grep "authentication failed" | awk '{print $X}' | sort | uniq -c
```

## Troubleshooting

### Redirect Loop

**Symptom**: Endless redirect between Authelia and Stisty

**Solutions**:
- Verify `rd` parameter in middleware address: `?rd=https://auth.yourdomain.com/`
- Check session domain matches: `session.domain: yourdomain.com`
- Ensure cookies are not blocked by browser
- Verify Traefik can reach Authelia container

```bash
# Test from Traefik container
docker exec traefik wget -O- http://authelia:9091/api/verify

# Check Authelia health
docker exec authelia wget -qO- http://localhost:9091/api/health
```

### 401 Unauthorized

**Symptom**: Always returns 401 even with valid credentials

**Solutions**:
- Check access control rules in `configuration.yml`
- Verify user is in correct group
- Review Authelia logs for specific error
- Ensure session storage (Redis) is working

```bash
# Check Redis connectivity
docker exec authelia redis-cli -h redis ping

# Verify access control
docker logs authelia | grep "access control"
```

### TOTP Setup Fails

**Symptom**: QR code doesn't scan or codes don't work

**Solutions**:
- Verify system time is synchronized (NTP)
- Check `totp.period` and `totp.skew` settings
- Try manual entry instead of QR code
- Ensure TOTP secret is properly stored in database

```bash
# Check system time
docker exec authelia date
docker exec redis date

# Verify time sync
timedatectl status
```

### Session Not Persisting

**Symptom**: Required to login on every page

**Solutions**:
- Check Redis is running and accessible
- Verify `session.domain` is set correctly
- Ensure HTTPS is used (cookies with `secure` flag)
- Clear browser cookies and try again

```bash
# Check Redis data
docker exec authelia-redis redis-cli KEYS "*"

# Verify session config
docker exec authelia cat /config/configuration.yml | grep -A 10 "session:"
```

### Headers Not Forwarded

**Symptom**: Application doesn't receive user information

**Solutions**:
- Verify `authResponseHeaders` in Traefik middleware
- Check application is reading correct headers (`Remote-User`, etc.)
- Review Traefik logs for header stripping

```bash
# Test headers from Traefik
curl -H "Host: genome.yourdomain.com" http://traefik/api/rawdata
```

## Security Best Practices

1. **Enable MFA** - Require two_factor policy for sensitive data
2. **Use Strong Secrets** - Generate 64+ character random secrets
3. **HTTPS Only** - Never serve Authelia over HTTP
4. **Restrict Networks** - Use IP whitelisting when possible
5. **Regular Updates** - Keep Authelia, Redis, and Traefik updated
6. **Audit Logs** - Monitor authentication attempts and failures
7. **Secure Redis** - Use password, bind to internal network only
8. **Email Validation** - Enable SMTP to verify user emails
9. **Session Timeouts** - Use short inactivity timeouts for sensitive apps
10. **Backup Secrets** - Securely store JWT, session, and encryption keys

## Complete Example

Full `docker-compose.yml` with Traefik + Authelia + Stisty:

```yaml
version: '3.8'

services:
  traefik:
    image: traefik:latest
    container_name: traefik
    restart: unless-stopped

    ports:
      - "80:80"
      - "443:443"

    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik:/etc/traefik
      - ./letsencrypt:/letsencrypt

    networks:
      - web

    command:
      - "--api.dashboard=true"
      - "--providers.docker=true"
      - "--providers.file.directory=/etc/traefik/dynamic"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--certificatesresolvers.letsencrypt.acme.email=admin@yourdomain.com"
      - "--certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json"
      - "--certificatesresolvers.letsencrypt.acme.httpchallenge.entrypoint=web"

  authelia:
    image: authelia/authelia:latest
    container_name: authelia
    restart: unless-stopped

    volumes:
      - ./authelia/config:/config

    networks:
      - web

    environment:
      - TZ=America/New_York

    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.authelia.rule=Host(`auth.yourdomain.com`)"
      - "traefik.http.routers.authelia.entrypoints=websecure"
      - "traefik.http.routers.authelia.tls.certresolver=letsencrypt"
      - "traefik.http.services.authelia.loadbalancer.server.port=9091"

      # Middleware definition
      - "traefik.http.middlewares.authelia.forwardauth.address=http://authelia:9091/api/verify?rd=https://auth.yourdomain.com/"
      - "traefik.http.middlewares.authelia.forwardauth.trustForwardHeader=true"
      - "traefik.http.middlewares.authelia.forwardauth.authResponseHeaders=Remote-User,Remote-Groups,Remote-Name,Remote-Email"

    depends_on:
      - redis

  redis:
    image: redis:alpine
    container_name: authelia-redis
    restart: unless-stopped

    volumes:
      - redis-data:/data

    networks:
      - web

  stisty-genome-web:
    build:
      context: .
      dockerfile: stisty-wasm/Dockerfile
    image: stisty-genome-web:latest
    container_name: stisty-genome-web
    restart: unless-stopped

    networks:
      - web

    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.stisty.rule=Host(`genome.yourdomain.com`)"
      - "traefik.http.routers.stisty.entrypoints=websecure"
      - "traefik.http.routers.stisty.tls.certresolver=letsencrypt"
      - "traefik.http.routers.stisty.middlewares=authelia"
      - "traefik.http.services.stisty.loadbalancer.server.port=80"

volumes:
  redis-data:

networks:
  web:
    external: true
```

## Migration from Other Auth Systems

### From Authentik

1. Export user list from Authentik
2. Generate password hashes for Authelia
3. Map groups to Authelia groups
4. Update Traefik middleware address
5. Test authentication flow
6. Switch over when verified

### From OAuth2 Proxy

1. Create users in Authelia
2. Map OAuth groups to Authelia groups
3. Update middleware configuration
4. Test before removing OAuth2 Proxy

## Support Resources

- **Authelia Documentation**: https://www.authelia.com/
- **GitHub**: https://github.com/authelia/authelia
- **Discord**: https://discord.authelia.com
- **Traefik Integration**: https://www.authelia.com/integration/proxies/traefik/

## Performance Tuning

### Redis Optimization

```yaml
# Redis configuration for production
redis:
  image: redis:alpine
  command: >
    redis-server
    --maxmemory 256mb
    --maxmemory-policy allkeys-lru
    --save ""
```

### Authelia Resources

```yaml
services:
  authelia:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
        reservations:
          cpus: '0.1'
          memory: 64M
```

## Backup and Recovery

**Backup critical files**:

```bash
# Backup configuration and database
tar -czf authelia-backup-$(date +%Y%m%d).tar.gz \
  authelia/config/configuration.yml \
  authelia/config/users_database.yml \
  authelia/config/db.sqlite3

# Backup Redis data (if persistence enabled)
docker exec authelia-redis redis-cli SAVE
cp /var/lib/docker/volumes/authelia_redis-data/_data/dump.rdb ./redis-backup.rdb
```

**Recovery**:

```bash
# Restore configuration
tar -xzf authelia-backup-YYYYMMDD.tar.gz

# Restart services
docker-compose restart authelia redis
```
