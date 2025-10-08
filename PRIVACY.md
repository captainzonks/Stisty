# Privacy Policy for Stisty Genome Analyzer

## Our Privacy Commitment

**Your genetic data belongs to you and only you.** Stisty is designed from the ground up with a privacy-first architecture that ensures your sensitive genetic information never leaves your control.

## How Stisty Protects Your Privacy

### 1. Client-Side Processing Only

**CLI Application:**
- Runs entirely on your local machine
- Reads genome files from your local filesystem
- Writes output files to your local filesystem
- Makes zero network connections
- No data transmission of any kind

**Web Application:**
- Runs entirely in your web browser using WebAssembly
- All computation happens client-side
- Zero server-side processing
- Zero data transmission to any server

### 2. No Data Collection

We do not collect, store, or transmit:
- ❌ Your genome data
- ❌ Analysis results
- ❌ File names or metadata
- ❌ Usage analytics tied to genetic data
- ❌ Cookies or tracking data related to genetic information
- ❌ Any personally identifiable information

### 3. Technical Architecture

**File Reading:**
- CLI: Direct filesystem access using Rust's standard library
- Web: Browser FileReader API (never transmitted over network)

**Processing:**
- CLI: Native Rust code on your machine
- Web: WebAssembly running in your browser's sandbox

**File Writing/Downloads:**
- CLI: Direct filesystem writes
- Web: Blob URLs created in browser memory, downloaded directly to your device

### 4. Verification

You can verify our privacy claims:

**Check Network Activity:**
1. Open your browser's Developer Tools (F12)
2. Go to the Network tab
3. Load a genome file in Stisty
4. Observe: Zero network requests containing genetic data

**Inspect Source Code:**
- All code is open source and available at: https://github.com/captainzonks/Stisty
- Search the codebase for network calls - you'll find none that transmit genetic data
- Review the WASM bindings - no external API calls

**Run Locally:**
- You can run the web application entirely offline
- No internet connection required after initial page load
- Still works with network disconnected

## Data You Control

### What Stays On Your Device:
✅ Original genome data files
✅ Analysis results
✅ Generated VCF files
✅ All intermediate processing data

### What's In Your Browser Memory (Web):
- Loaded genome data (cleared when page is closed)
- Analysis results (cleared when page is closed)
- Generated VCF content (cleared when page is closed)

**Important:** Browser memory is temporary and local to your device only.

## Third-Party Services

**We use ZERO third-party services that could access your genetic data:**
- ❌ No analytics platforms (Google Analytics, etc.)
- ❌ No error tracking services
- ❌ No CDNs serving genetic data
- ❌ No cloud storage
- ❌ No external APIs

## Open Source Transparency

Stisty is fully open source under MIT/Apache-2.0 dual license:
- Source code: https://github.com/captainzonks/Stisty
- You can audit every line of code
- You can build and run it yourself
- Community-reviewed for security and privacy

## Hosting Considerations

### Self-Hosting (Recommended for Maximum Privacy):
```bash
# Clone and run locally
git clone https://github.com/captainzonks/Stisty
cd Stisty/stisty-wasm
./build.sh
cd dist
python3 -m http.server 8080
# Access at http://localhost:8080
```

### Using Hosted Version:
- Even on a hosted website, your data never leaves your browser
- All processing is client-side via WebAssembly
- However, self-hosting provides additional peace of mind

## Your Rights

Because we never collect your data, you automatically have:
- ✅ Complete control over your data
- ✅ No need to request data deletion (we never have it)
- ✅ No need to opt-out (there's nothing to opt out of)
- ✅ No risk of data breaches (we don't store data)

## Contact

For privacy questions or concerns:
- GitHub Issues: https://github.com/captainzonks/Stisty/issues
- Security concerns: Please open a security advisory on GitHub

## Changes to Privacy Policy

Any changes to this privacy policy will be:
1. Documented in git history
2. Announced in release notes
3. Never weaken privacy protections without explicit user consent

## Legal Compliance

**GDPR Compliance:**
- We don't process personal data on servers → No GDPR data controller obligations
- Client-side processing → Data never leaves EU if you're in EU
- No data retention → No retention policy needed

**HIPAA:**
- Not applicable as we don't receive, store, or transmit Protected Health Information (PHI)
- Client-side processing means you maintain custody of your data

**Genetic Information Nondiscrimination Act (GINA):**
- We never access your genetic data, so discrimination is impossible
- Your genetic information stays under your exclusive control

---

## Summary

**Stisty's Privacy Model:**

```
Your Device
    ├─ Your genome file (stays here)
    ├─ Stisty processing (happens here)
    └─ Analysis results (stay here)

Internet
    └─ (Your genetic data never touches this)
```

**Bottom Line:** We can't breach your privacy because we never have access to your data in the first place.

---

*Last Updated: October 2025*
*Version: 1.0*