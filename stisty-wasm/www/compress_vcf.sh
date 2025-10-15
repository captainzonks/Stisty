#!/bin/bash
# Compress VCF files for Michigan Imputation Server
# Requires: bgzip (part of htslib/tabix package)
#
# Installation:
#   Ubuntu/Debian: sudo apt install tabix
#   macOS: brew install htslib
#   Windows: Use WSL or download from https://github.com/samtools/htslib

set -e

echo "==> Compressing VCF files with BGZIP"
echo ""

# Check if bgzip is installed
if ! command -v bgzip &> /dev/null; then
    echo "❌ Error: bgzip not found"
    echo ""
    echo "Please install bgzip first:"
    echo "  Ubuntu/Debian: sudo apt install tabix"
    echo "  macOS: brew install htslib"
    exit 1
fi

# Find all .vcf files in current directory
vcf_files=(*.vcf)

if [ ${#vcf_files[@]} -eq 0 ] || [ ! -f "${vcf_files[0]}" ]; then
    echo "❌ No VCF files found in current directory"
    echo "Please extract the ZIP file first and run this script from the same directory"
    exit 1
fi

echo "Found ${#vcf_files[@]} VCF file(s)"
echo ""

# Compress each VCF file
for vcf in "${vcf_files[@]}"; do
    if [ -f "$vcf" ]; then
        echo "Compressing: $vcf"
        bgzip -f "$vcf"
        echo "  ✅ Created: ${vcf}.gz"
    fi
done

echo ""
echo "==> Complete!"
echo ""
echo "Compressed files ready for Michigan Imputation Server:"
ls -lh *.vcf.gz
echo ""
echo "Next steps:"
echo "  1. Go to https://imputationserver.sph.umich.edu/index.html"
echo "  2. Create an account or sign in"
echo "  3. Upload the .vcf.gz files"
echo "  4. Select reference panel (e.g., HRC r1.1 2016 or 1000G Phase 3 v5)"
echo "  5. Run imputation"
