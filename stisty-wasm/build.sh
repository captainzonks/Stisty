#!/bin/bash
set -e

echo "🔨 Building Stisty WASM..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build WASM package
echo "🦀 Compiling Rust to WebAssembly..."
wasm-pack build --target web --out-dir pkg --release

# Copy files to dist directory
echo "📁 Creating distribution directory..."
rm -rf dist
mkdir -p dist

# Copy static files
cp www/index.html dist/
cp www/style.css dist/
cp www/app.js dist/

# Copy WASM files
cp pkg/stisty_wasm.js dist/
cp pkg/stisty_wasm_bg.wasm dist/

# Create a simple package.json for the dist
cat > dist/package.json << EOF
{
  "name": "stisty-web",
  "version": "0.1.0",
  "description": "Stisty Genome Analyzer Web Interface",
  "type": "module",
  "files": [
    "*.html",
    "*.css",
    "*.js",
    "*.wasm"
  ]
}
EOF

echo "✅ Build complete! Output in ./dist/"
echo ""
echo "To test locally, run:"
echo "  cd dist && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"