use anyhow::Result;
use std::path::PathBuf;
use stisty_lib::genetics::{GenomeAnalyzer, GenomeData};

fn main() -> Result<()> {

    // Get genome file path from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_23andme_file.txt>", args[0]);
        std::process::exit(1);
    }

    let genome_file = PathBuf::from(&args[1]);

    println!("Loading genome data from {:?}...", genome_file);
    let genome = GenomeData::from_file(&genome_file)?;

    println!("Successfully loaded genome data!\n");

    // Create analyzer
    let analyzer = GenomeAnalyzer::new(&genome);

    // Generate and display summary
    let summary = analyzer.generate_summary();
    println!("{}", summary.display());

    // Example: Look up specific SNPs of interest
    println!("\nExample SNP lookups:");
    println!("===================\n");

    // Look up some common SNPs (if they exist in the data)
    let example_rsids = ["rs548049170", "rs9326622", "rs3131972"];
    for rsid in &example_rsids {
        if let Some(snp) = genome.find_snp(rsid) {
            println!("Found {}: chr{} at position {} with genotype {} ({})",
                snp.rsid, snp.chromosome, snp.position, snp.genotype,
                if snp.is_heterozygous() { "heterozygous" } else { "homozygous" });
        }
    }

    // Example: Get all SNPs on chromosome 1
    let chr1_snps = genome.get_snps_by_chromosome("1");
    println!("\nTotal SNPs on Chromosome 1: {}", chr1_snps.len());

    Ok(())
}