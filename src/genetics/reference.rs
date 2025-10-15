/// Reference database for SNP annotations
///
/// Provides reference alleles, population frequencies, and other metadata
/// for known SNPs from dbSNP, gnomAD, and ClinVar databases.

use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};
use std::collections::HashMap;

/// SNP record from reference database (matches build_database.rs format)
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
#[repr(C)]
struct SnpRecord {
    /// Index into rsID string table
    rsid_index: u32,
    /// Chromosome (1-22=1-22, 23=X, 24=Y, 25=MT)
    chromosome: u8,
    /// Position on chromosome
    position: u32,
    /// Reference allele (2 bits) + Alternative allele (2 bits) + flags (4 bits)
    ref_alt_flags: u8,
    /// Minor allele frequency * 10000 (0-10000 = 0.00%-100.00%)
    maf: u16,
    /// Sample genotypes: 5 samples, 2 bits per allele = 8 bits per sample = 40 bits total
    /// Packed into u64: Each sample uses 8 bits (bits 0-1 = allele1, bits 2-3 = allele2)
    /// Encoding: 00=0, 01=1, 10=missing(.), 11=unused
    sample_genotypes: u64,
}

/// Complete reference database deserialized from binary format
#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct ReferenceDatabase {
    /// Version identifier
    version: String,
    /// Reference genome build
    build: String,
    /// Number of SNPs
    snp_count: usize,
    /// SNP records
    records: Vec<SnpRecord>,
    /// rsID string table (null-separated)
    rsid_table: String,
}

/// Reference information for a single SNP
#[derive(Debug, Clone)]
pub struct SnpReference {
    /// Reference allele (from reference genome)
    pub ref_allele: char,
    /// Alternative allele (common variant)
    pub alt_allele: char,
    /// Minor allele frequency (0.0 - 1.0)
    pub maf: f32,
    /// Chromosome
    pub chromosome: String,
    /// Position
    pub position: u32,
    /// Genotypes for 5 anonymous samples [samp1, samp2, samp3, samp4, samp5]
    /// Each is a string like "0/0", "0/1", "1/1", or "./."
    pub sample_genotypes: [String; 5],
}

impl ReferenceDatabase {
    /// Load reference database from brotli-compressed binary data
    #[cfg(target_arch = "wasm32")]
    pub async fn load_from_url(url: &str) -> Result<Self, String> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, Response};

        // Fetch the compressed database
        let opts = RequestInit::new();
        opts.set_method("GET");

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| format!("Failed to create request: {:?}", e))?;

        let window = web_sys::window().ok_or("No window object")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("Fetch failed: {:?}", e))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|_| "Response is not a Response object")?;

        // Get array buffer
        let array_buffer = JsFuture::from(resp.array_buffer().map_err(|e| format!("{:?}", e))?)
            .await
            .map_err(|e| format!("Failed to get array buffer: {:?}", e))?;

        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let compressed_data = uint8_array.to_vec();

        // Decompress with brotli
        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(
            &mut std::io::Cursor::new(compressed_data),
            &mut decompressed,
        )
        .map_err(|e| format!("Decompression failed: {:?}", e))?;

        // Deserialize with bincode 2.x
        bincode::decode_from_slice::<ReferenceDatabase, _>(&decompressed, bincode::config::standard())
            .map(|(db, _)| db)
            .map_err(|e| format!("Deserialization failed: {:?}", e))
    }

    /// Build an index for fast lookups by rsID
    pub fn build_index(&self) -> HashMap<String, usize> {
        let mut index = HashMap::new();
        let rsids: Vec<&str> = self.rsid_table.split('\0').filter(|s| !s.is_empty()).collect();

        for (idx, rsid) in rsids.iter().enumerate() {
            index.insert(rsid.to_string(), idx);
        }

        index
    }

    /// Look up reference information for an rsID
    pub fn lookup(&self, rsid: &str, index: &HashMap<String, usize>) -> Option<SnpReference> {
        let record_idx = index.get(rsid)?;
        let record = self.records.get(*record_idx)?;

        Some(SnpReference {
            ref_allele: decode_nucleotide((record.ref_alt_flags >> 6) & 0x03),
            alt_allele: decode_nucleotide((record.ref_alt_flags >> 4) & 0x03),
            maf: record.maf as f32 / 10000.0,
            chromosome: decode_chromosome(record.chromosome),
            position: record.position,
            sample_genotypes: decode_sample_genotypes(record.sample_genotypes),
        })
    }

    /// Get database statistics
    pub fn stats(&self) -> DatabaseStats {
        DatabaseStats {
            version: self.version.clone(),
            build: self.build.clone(),
            snp_count: self.snp_count,
            total_size: std::mem::size_of_val(&self.records[..]) + self.rsid_table.len(),
        }
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub version: String,
    pub build: String,
    pub snp_count: usize,
    pub total_size: usize,
}

/// Decode chromosome number to string
fn decode_chromosome(chr: u8) -> String {
    match chr {
        1..=22 => chr.to_string(),
        23 => "X".to_string(),
        24 => "Y".to_string(),
        25 => "MT".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Decode 2-bit nucleotide encoding to character
fn decode_nucleotide(code: u8) -> char {
    match code {
        0 => 'A',
        1 => 'C',
        2 => 'G',
        3 => 'T',
        _ => 'N',
    }
}

/// Decode sample genotypes from packed 64-bit integer
/// Format: 5 samples × 8 bits/sample = 40 bits
/// Each sample: bits 0-1 = allele1, bits 2-3 = allele2
/// Encoding: 00=0, 01=1, 10=missing(.), 11=unused
fn decode_sample_genotypes(packed: u64) -> [String; 5] {
    let mut genotypes = [String::new(), String::new(), String::new(), String::new(), String::new()];

    for i in 0..5 {
        let sample_bits = (packed >> (i * 8)) & 0xFF;
        let allele1 = sample_bits & 0x03;
        let allele2 = (sample_bits >> 2) & 0x03;

        let gt = match (allele1, allele2) {
            (2, 2) => "./.".to_string(),  // Missing
            (2, _) | (_, 2) => "./.".to_string(),  // Partial missing
            (a1, a2) => format!("{}/{}", a1, a2),  // 0/0, 0/1, 1/0, 1/1
        };

        genotypes[i] = gt;
    }

    genotypes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_nucleotide() {
        assert_eq!(decode_nucleotide(0), 'A');
        assert_eq!(decode_nucleotide(1), 'C');
        assert_eq!(decode_nucleotide(2), 'G');
        assert_eq!(decode_nucleotide(3), 'T');
        assert_eq!(decode_nucleotide(4), 'N');
    }

    #[test]
    fn test_decode_chromosome() {
        assert_eq!(decode_chromosome(1), "1");
        assert_eq!(decode_chromosome(22), "22");
        assert_eq!(decode_chromosome(23), "X");
        assert_eq!(decode_chromosome(24), "Y");
        assert_eq!(decode_chromosome(25), "MT");
    }
}
