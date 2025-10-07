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

## Future Enhancements

Planned features:
- Support for VCF format
- Trait association lookups using public databases
- Population genetics comparisons
- Variant effect prediction
- Export functionality for filtered SNP sets