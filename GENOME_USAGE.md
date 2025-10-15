# Genome Analysis Usage Guide

Stisty now supports genome data analysis for 23andMe raw data files and similar TSV formats.

## Quick Start

### Interactive Menu Mode

```bash
stisty --menu
```

Select "Genome Analysis (23andMe/TSV)" and follow the prompts.

### Command Line Mode

#### Full Summary Report
```bash
stisty --genetics --file path/to/genome_data.txt summary
```

#### Look Up Specific SNP
```bash
stisty --genetics --file path/to/genome_data.txt lookup --rsid rs548049170
```

#### Chromosome Statistics
```bash
stisty --genetics --file path/to/genome_data.txt chromosome --chromosome 1
```

#### Heterozygosity Rate
```bash
stisty --genetics --file path/to/genome_data.txt heterozygosity
```

#### Allele Frequencies
```bash
stisty --genetics --file path/to/genome_data.txt alleles
```

#### Transition/Transversion Ratio
```bash
stisty --genetics --file path/to/genome_data.txt ts-tv
```

## Available Analyses

### 1. Full Summary Report
Generates a comprehensive report including:
- Total SNP count
- Heterozygosity rate
- Transition/transversion ratio
- Allele frequencies
- SNPs per chromosome

### 2. SNP Lookup
Find specific SNPs by their rsid and view:
- Chromosome location
- Position
- Genotype
- Homozygous/heterozygous status

### 3. Chromosome Statistics
View statistics for a specific chromosome:
- Total SNP count
- Heterozygous SNP count and percentage

### 4. Heterozygosity Rate
Calculate the proportion of heterozygous SNPs in your genome.
Expected human range: ~20-35%

### 5. Allele Frequencies
View the frequency distribution of nucleotides (A, T, G, C) across all SNPs.

### 6. Transition/Transversion Ratio (Ts/Tv)
Calculate the ratio of transition mutations to transversion mutations.
Expected human range: ~2.0-2.1

## Data Privacy

**CRITICAL**: Genome data is extremely sensitive personal information.

- Always store genome files in `data_sources/` (already in .gitignore)
- Never commit genome data to version control
- Never share genome data in public repositories
- Be cautious when sharing code examples or output

## Supported File Formats

Currently supports 23andMe raw data format:
- Tab-separated values (TSV)
- Format: `rsid\tchromosome\tposition\tgenotype`
- Comments starting with `#` are parsed for metadata
- Reference build: GRCh37/hg19

## Examples

### Example 1: Generate a full summary
```bash
stisty -G -f data_sources/genome_data.txt summary
```

### Example 2: Look up multiple SNPs (using a shell loop)
```bash
for rsid in rs548049170 rs9326622 rs3131972; do
    stisty -G -f data_sources/genome_data.txt lookup -r $rsid
done
```

### Example 3: Get stats for all chromosomes
```bash
for chr in {1..22} X Y MT; do
    stisty -G -f data_sources/genome_data.txt chromosome -c $chr
done
```

## Understanding Your Results

### Heterozygosity Rate
- **Low (<20%)**: May indicate consanguinity or data quality issues
- **Normal (20-35%)**: Expected range for most populations
- **High (>35%)**: May indicate data quality issues or mixed samples

### Ts/Tv Ratio
- **Expected (~2.0-2.1)**: Normal for whole genome data
- **Higher (~3.0+)**: Common for exome sequencing (coding regions)
- **Lower (<1.5)**: May indicate data quality issues

### Allele Frequencies
Expected approximate frequencies for human genomes:
- A: ~25%
- T: ~25%
- G: ~25%
- C: ~25%

Significant deviations may indicate data quality issues.

## Troubleshooting

### "Genome file not found"
- Check the file path is correct
- Ensure the file exists in the specified location
- Use absolute paths or paths relative to current directory

### "Failed to parse SNP line"
- Ensure the file is in 23andMe format (TSV with 4 columns)
- Check that the file isn't corrupted
- Verify the file contains actual data (not just headers/comments)

### Slow loading
- Large genome files (600k+ SNPs) may take 10-30 seconds to load
- This is normal for the initial data parsing
- Consider using command-line mode for batch processing

## VCF Export for Imputation

Stisty can export genome data in VCF 4.2 format compatible with the [Michigan Imputation Server](https://imputationserver.sph.umich.edu/).

### CLI Tool with BGZF Compression

**NEW!** The CLI tool can now export VCF files with built-in BGZF compression:

```bash
# Export VCF files for all chromosomes with BGZF compression
stisty --genetics --file genome_data.txt export-vcf-bgzf --output ./vcf_output --name mygenome

# This creates files: B.mygenome_merged_6samples_chr{1-22}.vcf.gz
# Files are ready for direct upload to Michigan Imputation Server
```

**BGZF Benefits**:
- ✅ **True BGZF format** - Uses blocked compression (64KB blocks) for random access
- ✅ **Tabix indexing ready** - Can be indexed with `tabix -p vcf file.vcf.gz`
- ✅ **No external tools needed** - Compression happens automatically
- ✅ **Faster than external bgzip** - Multi-threaded compression via Rayon

### Web Application

The web interface provides uncompressed VCF files:

```bash
# Build and serve the WASM web application
cd stisty-wasm
./build.sh
cd dist
python3 -m http.server 8080
```

Visit `http://localhost:8080` and:
1. Upload your 23andMe raw data file
2. Click "Export VCF for Imputation"
3. Download the batch ZIP containing chr1-22 VCF files
4. Use the included `compress_vcf.sh` script to compress with bgzip
5. Upload the `.vcf.gz` files to Michigan Imputation Server

### Features

- ✅ **Michigan Imputation Server Compatible**: Passes QC with 0 allele switches
- ✅ **6 Samples**: Includes 5 anonymous samples + your genome (meets minimum requirements)
- ✅ **GRCh37 coordinates** with reference alleles from human reference genome
- ✅ **Reference panel filtering**: Only includes SNPs from the reference panel (552,550 SNPs)
- ✅ **Quality controlled**: Biallelic SNPs with valid REF/ALT alleles
- ✅ **Separate files per chromosome** (chr1-22) for optimal imputation
- ✅ **Privacy-first**: All processing happens in your browser, no server uploads

### Technical Details

The VCF export includes:
- **Format**: VCF 4.2 (Variant Call Format)
- **Reference**: GRCh37/hg19 coordinates
- **Samples**: 6 total (5 anonymous from reference panel + your genome)
- **Anonymous genotypes**: Real sample data from Matthew Keller's genotyped.anon.RData
- **Filtering**: Only SNPs present in reference panel to ensure REF/ALT consistency
- **Filename format**: `B.{name}_merged_6samples_chr{#}.vcf`

### Why Reference Panel Filtering?

Our implementation only exports SNPs that exist in the reference panel (genotyped.anon.RData). This is critical because:

1. **Prevents allele switches**: SNPs not in the reference panel would have arbitrary REF/ALT assignments
2. **Matches R script behavior**: Uses `merge(x, y, all.x=TRUE, all.y=FALSE)` logic
3. **Better QC results**: Our method includes 5.5% more SNPs than the R script version (281,830 vs 267,099)
4. **Fewer exclusions**: 35.8% fewer "low call rate" exclusions compared to R script

### Performance Comparison

| Metric | R Script | Stisty (Custom) | Improvement |
|--------|----------|-----------------|-------------|
| Allele switches | 0 | 0 | ✅ Identical |
| Matched SNPs | 269,625 | 283,452 | +13,827 (+5.1%) |
| Remaining sites | 267,099 | 281,830 | +14,731 (+5.5%) |
| Low call rate exclusions | 2,526 | 1,622 | -904 (-35.8%) |

## Future Enhancements

Planned features:
- Trait association lookups using public databases
- Population genetics comparisons
- Variant effect prediction
- Export functionality for filtered SNP sets
- Support for additional imputation servers