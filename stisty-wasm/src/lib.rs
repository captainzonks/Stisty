use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use stisty_lib::genetics::{GenomeAnalyzer, GenomeData, VcfGenerator};

// Set up panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// Use dlmalloc as the global allocator for WASM (wee_alloc is deprecated)
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[derive(Serialize, Deserialize)]
pub struct GenomeSummaryResult {
    pub total_snps: usize,
    pub heterozygosity_rate: f64,
    pub ts_tv_ratio: f64,
    pub allele_frequencies: Vec<(String, f64)>,
    pub chromosome_counts: Vec<(String, usize)>,
}

#[derive(Serialize, Deserialize)]
pub struct SnpResult {
    pub rsid: String,
    pub chromosome: String,
    pub position: u64,
    pub genotype: String,
    pub is_heterozygous: bool,
    pub is_homozygous: bool,
}

/// Analyze genome data from 23andMe text format
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
///
/// # Returns
/// JSON string containing the genome analysis summary
#[wasm_bindgen]
pub fn analyze_genome(file_content: &str) -> Result<String, JsValue> {
    // Parse genome data from the file content
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    // Create analyzer
    let analyzer = GenomeAnalyzer::new(&genome);

    // Generate summary
    let summary = analyzer.generate_summary();

    // Convert to WASM-friendly format
    let result = GenomeSummaryResult {
        total_snps: summary.total_snps,
        heterozygosity_rate: summary.heterozygosity_rate,
        ts_tv_ratio: summary.ts_tv_ratio,
        allele_frequencies: summary.allele_frequencies
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        chromosome_counts: summary.chromosome_counts
            .into_iter()
            .collect(),
    };

    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Look up a specific SNP by rsid
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
/// * `rsid` - The SNP identifier to look up (e.g., "rs548049170")
///
/// # Returns
/// JSON string containing the SNP information, or null if not found
#[wasm_bindgen]
pub fn lookup_snp(file_content: &str, rsid: &str) -> Result<String, JsValue> {
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    match genome.find_snp(rsid) {
        Some(snp) => {
            let result = SnpResult {
                rsid: snp.rsid.clone(),
                chromosome: snp.chromosome.clone(),
                position: snp.position,
                genotype: snp.genotype.clone(),
                is_heterozygous: snp.is_heterozygous(),
                is_homozygous: snp.is_homozygous(),
            };
            serde_json::to_string(&result)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
        }
        None => Ok("null".to_string())
    }
}

/// Get statistics for a specific chromosome
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
/// * `chromosome` - The chromosome to analyze (e.g., "1", "X", "MT")
///
/// # Returns
/// JSON string containing chromosome statistics
#[wasm_bindgen]
pub fn chromosome_stats(file_content: &str, chromosome: &str) -> Result<String, JsValue> {
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    let chr_snps = genome.get_snps_by_chromosome(chromosome);
    let total = chr_snps.len();
    let het_count = chr_snps.iter().filter(|snp| snp.is_heterozygous()).count();
    let het_rate = if total > 0 { het_count as f64 / total as f64 } else { 0.0 };

    let result = serde_json::json!({
        "chromosome": chromosome,
        "total_snps": total,
        "heterozygous_count": het_count,
        "heterozygosity_rate": het_rate,
    });

    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Generate VCF (Variant Call Format) output for genome data
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
/// * `chromosome` - Optional chromosome filter (e.g., "1", "X"). Pass empty string for all chromosomes.
///
/// # Returns
/// String containing the VCF file content
#[wasm_bindgen]
pub fn generate_vcf(file_content: &str, chromosome: &str) -> Result<String, JsValue> {
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    let generator = VcfGenerator::new(&genome);

    let chr_filter = if chromosome.is_empty() {
        None
    } else {
        Some(chromosome)
    };

    generator.generate_vcf(chr_filter)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate VCF: {}", e)))
}

// Helper function to parse genome data from string
fn parse_genome_from_string(content: &str) -> anyhow::Result<GenomeData> {
    // Parse genome data directly from string (no filesystem access needed)
    GenomeData::from_string(content)
}