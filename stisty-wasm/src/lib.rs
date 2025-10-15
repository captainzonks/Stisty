use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use stisty_lib::genetics::{GenomeAnalyzer, GenomeData, VcfGenerator, ReferenceDatabase};
use std::cell::RefCell;

// Thread-local storage for the reference database
thread_local! {
    static REF_DB: RefCell<Option<ReferenceDatabase>> = RefCell::new(None);
    static REF_INDEX: RefCell<Option<std::collections::HashMap<String, usize>>> = RefCell::new(None);
}

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

/// Generate VCF with reference database support (requires reference database to be loaded)
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
/// * `chromosome` - Optional chromosome filter (e.g., "1", "X"). Pass empty string for all chromosomes.
///
/// # Returns
/// String containing the VCF file content with proper REF/ALT alleles from reference genome
#[wasm_bindgen]
pub fn generate_vcf_with_reference(file_content: &str, chromosome: &str) -> Result<String, JsValue> {
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    // Get reference database and index from thread-local storage
    let result = REF_DB.with(|db_cell| {
        REF_INDEX.with(|idx_cell| {
            let db_opt = db_cell.borrow();
            let idx_opt = idx_cell.borrow();

            match (db_opt.as_ref(), idx_opt.as_ref()) {
                (Some(db), Some(idx)) => {
                    let generator = VcfGenerator::with_reference(&genome, db, idx);

                    let chr_filter = if chromosome.is_empty() {
                        None
                    } else {
                        Some(chromosome)
                    };

                    generator.generate_vcf(chr_filter)
                },
                _ => Err(anyhow::anyhow!("Reference database not loaded. Call load_reference_database() first."))
            }
        })
    });

    result.map_err(|e| JsValue::from_str(&format!("Failed to generate VCF: {}", e)))
}

/// Generate batch VCF files for chromosomes 1-22 with reference database support
///
/// # Arguments
/// * `file_content` - The raw text content from a 23andMe genome file
///
/// # Returns
/// JSON string containing a map of chromosome names to VCF content
/// Format: { "1": "vcf content...", "2": "vcf content...", ... }
#[wasm_bindgen]
pub fn generate_batch_vcf_with_reference(file_content: &str) -> Result<String, JsValue> {
    let genome = parse_genome_from_string(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse genome data: {}", e)))?;

    // Get reference database and index from thread-local storage
    let result = REF_DB.with(|db_cell| {
        REF_INDEX.with(|idx_cell| {
            let db_opt = db_cell.borrow();
            let idx_opt = idx_cell.borrow();

            match (db_opt.as_ref(), idx_opt.as_ref()) {
                (Some(db), Some(idx)) => {
                    let generator = VcfGenerator::with_reference(&genome, db, idx);
                    generator.generate_batch_vcf()
                },
                _ => Err(anyhow::anyhow!("Reference database not loaded. Call load_reference_database() first."))
            }
        })
    });

    let vcf_files = result.map_err(|e| JsValue::from_str(&format!("Failed to generate batch VCF: {}", e)))?;

    // Convert to JSON for JavaScript
    serde_json::to_string(&vcf_files)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize VCF files: {}", e)))
}

// Helper function to parse genome data from string
fn parse_genome_from_string(content: &str) -> anyhow::Result<GenomeData> {
    // Parse genome data directly from string (no filesystem access needed)
    GenomeData::from_string(content)
}

/// Load the reference database from URL
///
/// # Arguments
/// * `url` - URL to the brotli-compressed reference database file
///
/// # Returns
/// Promise that resolves when database is loaded
#[wasm_bindgen]
pub async fn load_reference_database(url: &str) -> Result<String, JsValue> {
    // Load database
    let db = ReferenceDatabase::load_from_url(url)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to load reference database: {}", e)))?;

    // Build index
    let index = db.build_index();
    let stats = db.stats();

    // Store in thread-local storage
    REF_DB.with(|cell| {
        *cell.borrow_mut() = Some(db);
    });

    REF_INDEX.with(|cell| {
        *cell.borrow_mut() = Some(index);
    });

    // Return stats as JSON
    let result = serde_json::json!({
        "version": stats.version,
        "build": stats.build,
        "snp_count": stats.snp_count,
        "size_bytes": stats.total_size,
    });

    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Look up reference information for an rsID
///
/// # Arguments
/// * `rsid` - The SNP identifier to look up
///
/// # Returns
/// JSON string containing reference allele, alt allele, MAF, etc. or null if not found
#[wasm_bindgen]
pub fn lookup_reference(rsid: &str) -> Result<String, JsValue> {
    let result = REF_DB.with(|db_cell| {
        let db_ref = db_cell.borrow();
        let db = db_ref.as_ref().ok_or("Reference database not loaded")?;

        REF_INDEX.with(|index_cell| {
            let index_ref = index_cell.borrow();
            let index = index_ref.as_ref().ok_or("Reference index not built")?;

            match db.lookup(rsid, index) {
                Some(snp_ref) => {
                    let json = serde_json::json!({
                        "rsid": rsid,
                        "chromosome": snp_ref.chromosome,
                        "position": snp_ref.position,
                        "ref_allele": snp_ref.ref_allele.to_string(),
                        "alt_allele": snp_ref.alt_allele.to_string(),
                        "maf": snp_ref.maf,
                    });
                    Ok(serde_json::to_string(&json)
                        .map_err(|e| format!("Serialization error: {}", e))?)
                }
                None => Ok("null".to_string())
            }
        })
    });

    result.map_err(|e: String| JsValue::from_str(&e))
}