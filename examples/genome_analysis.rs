use anyhow::Result;
use std::path::PathBuf;
use stisty_lib::genetics::{GenomeAnalyzer, GenomeData, VcfGenerator};

fn main() -> Result<()> {

    // Get genome file path from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_23andme_file.txt> [--vcf <chromosome>] [--output <file.vcf>]", args[0]);
        std::process::exit(1);
    }

    let genome_file = PathBuf::from(&args[1]);

    // Parse command line options
    let mut vcf_mode = false;
    let mut vcf_chromosome: Option<String> = None;
    let mut output_file: Option<PathBuf> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--vcf" => {
                vcf_mode = true;
                if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    vcf_chromosome = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    println!("Loading genome data from {:?}...", genome_file);
    let genome = GenomeData::from_file(&genome_file)?;

    println!("Successfully loaded genome data!\n");

    // If VCF mode is enabled, generate VCF and exit
    if vcf_mode {
        println!("Generating VCF output...");
        let generator = VcfGenerator::new(&genome);
        let vcf_content = generator.generate_vcf(vcf_chromosome.as_deref())?;

        if let Some(output_path) = output_file {
            std::fs::write(&output_path, vcf_content)?;
            println!("VCF file written to {:?}", output_path);
        } else {
            print!("{}", vcf_content);
        }

        return Ok(());
    }

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