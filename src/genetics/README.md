# Genetics Module

Analysis tools for genome data from 23andMe and similar services.

## Features

- **SNP Data Model**: Parse and store Single Nucleotide Polymorphism data
- **23andMe Format Support**: Direct import from 23andMe raw data files (tab-separated format)
- **Genome Analysis**: Calculate various genetic metrics and statistics
- **SNP Lookup**: Find specific SNPs by rsid
- **Chromosome Filtering**: Query SNPs by chromosome

## Usage

### Loading Genome Data

```rust
use stisty_lib::genetics::GenomeData;
use std::path::Path;

let genome = GenomeData::from_file(Path::new("path/to/genome_data.txt"))?;
```

### Analyzing Genome Data

```rust
use stisty_lib::genetics::GenomeAnalyzer;

let analyzer = GenomeAnalyzer::new(&genome);
let summary = analyzer.generate_summary();

println!("{}", summary.display());
```

### Querying SNPs

```rust
// Find a specific SNP
if let Some(snp) = genome.find_snp("rs548049170") {
    println!("Genotype: {}", snp.genotype);
    println!("Is heterozygous: {}", snp.is_heterozygous());
}

// Get all SNPs on a chromosome
let chr1_snps = genome.get_snps_by_chromosome("1");
println!("SNPs on Chr 1: {}", chr1_snps.len());
```

### Calculating Metrics

```rust
// Heterozygosity rate
let het_rate = genome.heterozygosity_rate();

// Allele frequencies
let freqs = analyzer.calculate_allele_frequencies();

// Transition/transversion ratio
let ts_tv = analyzer.transition_transversion_ratio();

// SNP counts per chromosome
let counts = genome.chromosome_counts();
```

## Running the Example

```bash
cargo run --example genome_analysis -- path/to/your/genome_file.txt
```

## Data Privacy

**IMPORTANT**: Genome data files contain highly sensitive personal information. Always:
- Keep genome data files in the `data_sources/` directory (already in .gitignore)
- Never commit genome data to version control
- Never hardcode file paths to personal genome data
- Be cautious when sharing code or examples

## Supported File Formats

Currently supports 23andMe raw data format:
- Tab-separated values (TSV)
- Four columns: rsid, chromosome, position, genotype
- Comment lines starting with `#` are parsed for metadata
- Reference build: GRCh37/hg19 (build 37)

## Future Enhancements

Potential additions:
- Support for other genome data providers (AncestryDNA, etc.)
- Trait association lookups using public databases
- Variant effect prediction
- Population genetics comparisons
- Export to VCF format