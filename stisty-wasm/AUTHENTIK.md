# Authentik Integration Guide

This guide covers integrating Stisty Genome Analyzer with Authentik for enterprise authentication.

## Why Authentik?

- 🔐 **SSO Support** - OIDC, SAML, OAuth2
- 👥 **User Management** - Groups, roles, permissions
- 🔑 **MFA** - TOTP, WebAuthn, SMS
- 📊 **Audit Logs** - Track access to sensitive genetic data
- 🎯 **Fine-grained Access** - Policy-based access control

## Architecture

```
User Browser
     ↓
Traefik (Reverse Proxy)
     ↓
Authentik (Forward Auth)
     ↓ (authorized)
Stisty Container (WASM App)
```

## Prerequisites

- Traefik reverse proxy running
- Authentik instance deployed
- Docker network connecting all services

## Setup Steps

### 1. Create Provider in Authentik

1. Navigate to **Applications → Providers**
2. Click **Create**
3. Select **Proxy Provider**
4. Configure:
   - **Name**: `Stisty Genome Analyzer`
   - **Authorization flow**: `default-provider-authorization-implicit-consent`
   - **Type**: `Forward auth (single application)`
   - **External host**: `https://genome.yourdomain.com`
   - **Token validity**: `hours=24` (or as needed)

5. Click **Finish**

### 2. Create Application

1. Navigate to **Applications → Applications**
2. Click **Create**
3. Configure:
   - **Name**: `Stisty Genome Analyzer`
   - **Slug**: `stisty-genome`
   - **Provider**: Select the provider created above
   - **Policy engine mode**: `any`
   - **UI settings**:
     - **Launch URL**: `https://genome.yourdomain.com`
     - **Icon**: Upload a genome/DNA icon (optional)

4. Click **Create**

### 3. Create Access Policy (Optional but Recommended)

For restricting access to specific groups:

1. Navigate to **Customization → Policies**
2. Click **Create → Expression Policy**
3. Configure:
   - **Name**: `Genome Analyzer Access`
   - **Expression**:
     ```python
     # Allow only specific group
     return ak_is_group_member(request.user, name="genome-users")

     # OR allow multiple groups
     return ak_is_group_member(request.user, name="genome-users") or \
            ak_is_group_member(request.user, name="admins")

     # OR allow based on email domain
     return request.user.email.endswith("@yourdomain.com")
     ```

4. Bind policy to application:
   - Go to your Stisty application
   - **Policy Bindings** tab
   - Add the policy you just created

### 4. Create Outpost (if not using default)

If using a dedicated outpost:

1. Navigate to **Applications → Outposts**
2. Click **Create**
3. Configure:
   - **Name**: `Embedded Outpost` (or custom name)
   - **Type**: `Proxy`
   - **Integration**: `Local Docker connection` (or your setup)
   - **Applications**: Select your Stisty application

4. Deploy outpost (if external):
   ```bash
   docker run -d \
     --name authentik-proxy \
     --network traefik_net \
     -e AUTHENTIK_HOST=https://auth.yourdomain.com \
     -e AUTHENTIK_TOKEN=your_token_here \
     ghcr.io/goauthentik/proxy:latest
   ```

### 5. Configure Traefik Middleware

**Option A: File Provider** (`traefik/dynamic/middlewares.yml`):

```yaml
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
          - X-authentik-uid
        authRequestHeaders:
          - X-Forwarded-Host
          - X-Forwarded-Uri
```

**Option B: Docker Labels** (in Traefik's static config or separate file):

```yaml
# In your Traefik docker-compose or config
labels:
  - "traefik.enable=true"
  - "traefik.http.middlewares.authentik.forwardauth.address=http://authentik-server:9000/outpost.goauthentik.io/auth/traefik"
  - "traefik.http.middlewares.authentik.forwardauth.trustForwardHeader=true"
  - "traefik.http.middlewares.authentik.forwardauth.authResponseHeaders=X-authentik-username,X-authentik-groups,X-authentik-email,X-authentik-name"
```

### 6. Configure Stisty Service

**Update docker-compose.yml**:

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

      # Apply Authentik middleware
      - "traefik.http.routers.stisty.middlewares=authentik@file"

      # Service configuration
      - "traefik.http.services.stisty.loadbalancer.server.port=80"

networks:
  traefik_net:
    external: true
```

**Or using dynamic file** (`traefik/dynamic/stisty.yml`):

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
        - authentik@file

  services:
    stisty-genome:
      loadBalancer:
        servers:
          - url: "http://stisty-genome-web:80"
```

### 7. Test Authentication Flow

1. Navigate to `https://genome.yourdomain.com`
2. Should redirect to Authentik login
3. Login with your credentials
4. Complete MFA if enabled
5. Redirect back to Stisty application
6. Verify application loads

## Advanced Configuration

### Multi-Factor Authentication

Enable MFA for genome analyzer access:

1. Navigate to **Flows & Stages → Flows**
2. Edit `default-authentication-flow`
3. Add MFA stages:
   - TOTP Authenticator Validation Stage
   - WebAuthn Authenticator Validation Stage

### Session Management

Configure session timeout:

1. In your Provider settings:
   - **Access code validity**: `minutes=1`
   - **Access token validity**: `hours=24`
   - **Refresh token validity**: `days=30`

### Audit Logging

Track genome analyzer access:

1. Navigate to **Events → Logs**
2. Filter by application: `stisty-genome`
3. Export logs if needed for compliance

### Group-Based Access

Create specific groups:

```bash
# In Authentik UI: Directory → Groups
- Create group: "genome-users"
- Add users to group
- Apply policy (see step 3 above)
```

### IP Whitelisting

Combine with IP-based policy:

```python
# In policy expression
from ipaddress import ip_address, ip_network

allowed_networks = [
    ip_network("192.168.1.0/24"),
    ip_network("10.0.0.0/8"),
]

user_ip = ip_address(request.http_request.META.get("HTTP_X_FORWARDED_FOR", "").split(",")[0])

return any(user_ip in network for network in allowed_networks)
```

## Troubleshooting

### Authentication Loop

**Symptom**: Endless redirect between Authentik and Stisty

**Solutions**:
- Verify `External host` matches exactly (including https://)
- Check Traefik middleware is applied correctly
- Ensure `trustForwardHeader: true` is set
- Verify network connectivity between containers

```bash
# Test from Traefik container
docker exec traefik wget -O- http://authentik-server:9000/outpost.goauthentik.io/auth/traefik
```

### 401 Unauthorized

**Symptom**: Always returns 401 even with valid credentials

**Solutions**:
- Check Provider is correctly bound to Application
- Verify outpost is running and healthy
- Check policy bindings allow your user
- Review Authentik logs for errors

```bash
# Check Authentik logs
docker logs authentik-server --tail 100
```

### Cookie Issues

**Symptom**: Authentication works but session doesn't persist

**Solutions**:
- Verify domain matches cookie domain
- Check `SameSite` cookie settings
- Ensure HTTPS is enforced
- Clear browser cookies and try again

### Headers Not Forwarded

**Symptom**: Application doesn't receive user info

**Solutions**:
- Verify `authResponseHeaders` in middleware
- Check nginx doesn't strip headers
- Add headers to nginx config if needed:

```nginx
location / {
    proxy_set_header X-authentik-username $http_x_authentik_username;
    proxy_set_header X-authentik-email $http_x_authentik_email;
    # ... other headers
}
```

## Security Best Practices

1. **Enable MFA** - Require for all genome analyzer access
2. **Use Groups** - Don't grant access to all users
3. **Short Sessions** - Consider 1-4 hour token validity for sensitive data
4. **Audit Logs** - Regular review of access patterns
5. **Policy Enforcement** - Use expression policies for fine-grained control
6. **IP Restrictions** - Combine with IP whitelisting when possible
7. **Regular Updates** - Keep Authentik and outpost updated

## Example Full Setup

Complete docker-compose.yml with Traefik + Authentik + Stisty:

```yaml
version: '3.8'

services:
  traefik:
    image: traefik:latest
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik:/etc/traefik
    networks:
      - web

  authentik-server:
    image: ghcr.io/goauthentik/server:latest
    restart: unless-stopped
    command: server
    environment:
      AUTHENTIK_SECRET_KEY: your-secret-key
      AUTHENTIK_ERROR_REPORTING__ENABLED: "false"
    volumes:
      - ./authentik/media:/media
      - ./authentik/templates:/templates
    networks:
      - web
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.authentik.rule=Host(`auth.yourdomain.com`)"

  stisty-genome-web:
    build: .
    networks:
      - web
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.stisty.rule=Host(`genome.yourdomain.com`)"
      - "traefik.http.routers.stisty.middlewares=authentik@file"

networks:
  web:
    external: true
```

## Support

For Authentik-specific issues:
- [Authentik Documentation](https://goauthentik.io/docs/)
- [Authentik Discord](https://goauthentik.io/discord)

For Stisty integration issues:
- [GitHub Issues](https://github.com/captainzonks/Stisty/issues)