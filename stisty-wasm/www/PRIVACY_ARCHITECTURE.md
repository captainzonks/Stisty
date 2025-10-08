# Privacy Architecture - Technical Details

## How Stisty Ensures Client-Side Processing

This document provides technical details about Stisty's privacy architecture for developers and security auditors.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        User's Browser                        │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              JavaScript Application                     │ │
│  │  - File reading (FileReader API)                       │ │
│  │  - UI rendering                                        │ │
│  │  - Blob URL creation (downloads)                       │ │
│  └────────────────────┬───────────────────────────────────┘ │
│                       │                                      │
│                       ▼                                      │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          WebAssembly Module (Rust)                     │ │
│  │  - Genome parsing                                      │ │
│  │  - Statistical analysis                                │ │
│  │  - VCF generation                                      │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  All data stays in browser memory                           │
│  No network transmission                                     │
└─────────────────────────────────────────────────────────────┘

                              ❌ NO CONNECTION ❌

┌─────────────────────────────────────────────────────────────┐
│                      External Servers                        │
│                     (Never accessed)                         │
└─────────────────────────────────────────────────────────────┘
```

## Code Flow Analysis

### 1. File Upload (app.js:25-52)

```javascript
fileInput.addEventListener('change', async (event) => {
    const file = event.target.files[0];  // File object from browser

    // PRIVACY: file.text() is a browser API that reads the file
    // from local disk into memory. No network involved.
    const text = await file.text();

    // Store in JavaScript variable (browser memory only)
    genomeData = text;

    // Call WASM function (executes in browser)
    const resultJson = analyze_genome(text);
});
```

**Privacy guarantees:**
- `file.text()` is a Web API that reads files locally
- No `fetch()`, `XMLHttpRequest`, or `navigator.sendBeacon()` calls
- Data stored in JavaScript closure (memory only)

### 2. WASM Processing (src/lib.rs)

```rust
#[wasm_bindgen]
pub fn analyze_genome(file_content: &str) -> Result<String, JsValue> {
    // Parse genome data from string (in-memory only)
    let genome = parse_genome_from_string(file_content)?;

    // Analyze (pure computation, no I/O)
    let analyzer = GenomeAnalyzer::new(&genome);
    let summary = analyzer.generate_summary();

    // Return JSON (stays in WASM/JS boundary)
    serde_json::to_string(&result)
}
```

**Privacy guarantees:**
- No network-related Rust crates used
- No file system access (WASM sandbox)
- Pure computation functions only
- No external system calls

### 3. VCF Generation (src/genetics/vcf.rs)

```rust
pub fn generate_vcf(&self, chromosome: Option<&str>) -> Result<String> {
    let mut output = String::new();

    // Write VCF header
    self.write_header(&mut output)?;

    // Write variants
    for snp in sorted_snps {
        self.write_variant_line(&mut output, snp)?;
    }

    // Return string (in-memory)
    Ok(output)
}
```

**Privacy guarantees:**
- Builds string in memory
- No file I/O (WASM can't access filesystem)
- Returns to JavaScript as string

### 4. File Download (app.js:321-346)

```javascript
downloadVcfButton.addEventListener('click', () => {
    // Create Blob from string (browser memory)
    const blob = new Blob([vcfData], { type: 'text/plain' });

    // Create object URL (local to browser)
    const url = URL.createObjectURL(blob);

    // Trigger download via anchor element
    const a = document.createElement('a');
    a.href = url;  // blob:http://... URL (local)
    a.download = `genome${chrSuffix}.vcf`;
    a.click();

    // Clean up memory
    URL.revokeObjectURL(url);
});
```

**Privacy guarantees:**
- `Blob` creates data in browser heap memory
- `URL.createObjectURL()` creates local reference (blob: URL)
- Download happens directly from browser memory to disk
- No network transmission

## Security Audit Checklist

### ✅ Verify No Network Calls

```bash
# Search for network-related code
cd Stisty
grep -r "fetch(" stisty-wasm/www/
grep -r "XMLHttpRequest" stisty-wasm/www/
grep -r "axios" stisty-wasm/www/
grep -r "navigator.sendBeacon" stisty-wasm/www/

# Should return NO results containing genetic data transmission
```

### ✅ Verify WASM Dependencies

```bash
# Check Cargo.toml for network crates
cd Stisty/stisty-wasm
cat Cargo.toml | grep -E "(reqwest|hyper|curl|http)"

# Should return NO results
```

### ✅ Verify Browser APIs

All browser APIs used:
- ✅ `FileReader.text()` - Local file reading
- ✅ `Blob()` - Memory-only data containers
- ✅ `URL.createObjectURL()` - Local blob URLs
- ✅ WebAssembly APIs - Sandboxed execution

APIs **NOT** used:
- ❌ `fetch()` - Would enable network requests
- ❌ `XMLHttpRequest` - Would enable network requests
- ❌ `navigator.sendBeacon()` - Would enable analytics
- ❌ `localStorage`/`sessionStorage` - Would persist data

### ✅ Verify WebAssembly Sandbox

WASM security features:
- Cannot access filesystem
- Cannot make network calls
- Cannot access system resources
- Runs in browser security sandbox
- Memory isolated from host

## Network Traffic Analysis

### Expected Network Requests

When running the web app, you should see:

**Initial Load:**
```
GET /index.html
GET /style.css
GET /app.js
GET /stisty_wasm.js
GET /stisty_wasm_bg.wasm
```

**During Genetic Data Processing:**
```
(No requests)
```

### Monitoring Network Traffic

**Chrome DevTools:**
1. Open DevTools (F12)
2. Go to Network tab
3. Click "Clear" to clear existing requests
4. Upload genome file
5. Observe: No new network requests

**Firefox DevTools:**
1. Open DevTools (F12)
2. Go to Network tab
3. Click trash icon to clear
4. Upload genome file
5. Observe: No new network requests

**Command Line (tcpdump):**
```bash
# Monitor network traffic while using app
sudo tcpdump -i any -n 'port 80 or port 443'

# Should see no traffic after initial page load
```

## WebAssembly Capabilities

### What WASM Can Do:
✅ Pure computation (math, string processing)
✅ Access linear memory (isolated sandbox)
✅ Call imported JavaScript functions
✅ Return results to JavaScript

### What WASM Cannot Do:
❌ Access files directly
❌ Make network requests directly
❌ Access DOM directly
❌ Persist data
❌ Access system resources

## Code Review Guidelines

When reviewing changes, reject PRs that:

❌ Add `fetch()` or `XMLHttpRequest` calls
❌ Add analytics libraries
❌ Add external script tags
❌ Add localStorage/sessionStorage for genetic data
❌ Add network-capable Rust crates
❌ Add telemetry or error reporting services

✅ Accept PRs that:
- Improve client-side algorithms
- Fix bugs in computation
- Enhance UI/UX
- Improve documentation

## Deployment Considerations

### Self-Hosted Deployment (Recommended)

```bash
# Run entirely locally
cd Stisty/stisty-wasm
./build.sh
cd dist
python3 -m http.server 8080

# Or use any static file server
# All processing stays client-side
```

### Public Hosting

Even when hosted publicly (e.g., GitHub Pages):
- Initial page load comes from server (HTML, CSS, JS, WASM)
- All genetic data processing happens client-side
- No genetic data transmitted to hosting server

**Server logs will show:**
```
GET /index.html - OK
GET /app.js - OK
GET /stisty_wasm_bg.wasm - OK
(No genetic data in any request)
```

## Threat Model

### Protected Against:

✅ **Man-in-the-Middle Attacks**
- Genetic data never transmitted, so nothing to intercept

✅ **Server Breaches**
- Server never receives genetic data

✅ **Database Leaks**
- No database stores genetic data

✅ **Insider Threats**
- Developers/admins never have access to user genetic data

✅ **Subpoenas/Legal Requests**
- No data to hand over

### Not Protected Against:

⚠️ **Client-Side Attacks**
- Malicious browser extensions could access data in memory
- User's device compromise could expose data
- **Mitigation:** User controls their environment

⚠️ **Screen Recording**
- Visible results could be recorded
- **Mitigation:** User controls their display

## Verification Commands

```bash
# Clone and audit
git clone https://github.com/captainzonks/Stisty
cd Stisty

# Check for network code in web app
grep -r "fetch\|XMLHttpRequest\|axios" stisty-wasm/www/

# Check for network crates in WASM
cd Stisty/stisty-wasm
cat Cargo.toml

# Build and inspect
./build.sh

# Run locally and monitor network
cd dist
python3 -m http.server 8080 &
# Open browser to http://localhost:8080
# Use DevTools Network tab
```

## Summary

**Privacy by Architecture:**
- Not privacy by policy (we promise not to look)
- Not privacy by security (we encrypt it)
- **Privacy by impossibility** (we literally cannot access your data)

The architecture makes it **technically impossible** for us to access your genetic data because it never leaves your device.

---

*For questions about this architecture, please open an issue on GitHub.*