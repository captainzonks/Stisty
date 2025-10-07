use anyhow::{Context, Result};
use log::info;
use std::path::Path;

/// Represents a Single Nucleotide Polymorphism (SNP) from genome data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SNP {
    /// SNP identifier (rsid or internal id)
    pub rsid: String,
    /// Chromosome (1-22, X, Y, or MT)
    pub chromosome: String,
    /// Position on the chromosome
    pub position: u64,
    /// Genotype (e.g., AA, TT, GG, CC, AT, etc.)
    pub genotype: String,
}

impl SNP {
    pub fn new(rsid: String, chromosome: String, position: u64, genotype: String) -> Self {
        Self {
            rsid,
            chromosome,
            position,
            genotype,
        }
    }

    /// Parse a line from 23andMe data format
    /// Format: rsid\tchromosome\tposition\tgenotype
    pub fn from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            anyhow::bail!("Invalid SNP line format: expected 4 tab-separated fields, got {}", parts.len());
        }

        let rsid = parts[0].to_string();
        let chromosome = parts[1].to_string();
        let position = parts[2]
            .parse::<u64>()
            .context("Failed to parse position as u64")?;
        let genotype = parts[3].to_string();

        Ok(Self::new(rsid, chromosome, position, genotype))
    }

    /// Check if this SNP is heterozygous (two different alleles)
    pub fn is_heterozygous(&self) -> bool {
        if self.genotype.len() != 2 {
            return false;
        }
        let chars: Vec<char> = self.genotype.chars().collect();
        chars[0] != chars[1]
    }

    /// Check if this SNP is homozygous (two identical alleles)
    pub fn is_homozygous(&self) -> bool {
        if self.genotype.len() != 2 {
            return false;
        }
        let chars: Vec<char> = self.genotype.chars().collect();
        chars[0] == chars[1]
    }
}

/// Container for genome data from 23andMe
#[derive(Debug, Clone, Default)]
pub struct GenomeData {
    /// Vector of SNPs
    pub snps: Vec<SNP>,
    /// Metadata from file header
    pub metadata: GenomeMetadata,
}

/// Metadata extracted from 23andMe file header
#[derive(Debug, Clone, Default)]
pub struct GenomeMetadata {
    pub file_id: Option<String>,
    pub signature: Option<String>,
    pub timestamp: Option<String>,
    pub build: String,
}

impl GenomeData {
    pub fn new() -> Self {
        Self {
            snps: Vec::new(),
            metadata: GenomeMetadata {
                file_id: None,
                signature: None,
                timestamp: None,
                build: String::from("GRCh37/hg19"),
            },
        }
    }

    /// Import genome data from 23andMe text file
    pub fn from_file(file_path: &Path) -> Result<Self> {
        info!("Importing 23andMe genome data from {:?}", file_path);

        let content = std::fs::read_to_string(file_path)
            .context("Failed to read genome data file")?;

        let mut genome_data = Self::new();
        let mut snp_count = 0;

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Parse metadata from comments
            if trimmed.starts_with('#') {
                if trimmed.starts_with("# file_id:") {
                    genome_data.metadata.file_id = Some(trimmed.trim_start_matches("# file_id:").trim().to_string());
                } else if trimmed.starts_with("# signature:") {
                    genome_data.metadata.signature = Some(trimmed.trim_start_matches("# signature:").trim().to_string());
                } else if trimmed.starts_with("# timestamp:") {
                    genome_data.metadata.timestamp = Some(trimmed.trim_start_matches("# timestamp:").trim().to_string());
                }
                continue;
            }

            // Skip the header line
            if trimmed.starts_with("rsid") {
                continue;
            }

            // Parse SNP data
            match SNP::from_line(trimmed) {
                Ok(snp) => {
                    genome_data.snps.push(snp);
                    snp_count += 1;
                }
                Err(e) => {
                    log::warn!("Failed to parse SNP line: {} - Error: {}", trimmed, e);
                }
            }
        }

        info!("Successfully imported {} SNPs from genome data", snp_count);
        Ok(genome_data)
    }

    /// Get all SNPs on a specific chromosome
    pub fn get_snps_by_chromosome(&self, chromosome: &str) -> Vec<&SNP> {
        self.snps
            .iter()
            .filter(|snp| snp.chromosome == chromosome)
            .collect()
    }

    /// Find a specific SNP by rsid
    pub fn find_snp(&self, rsid: &str) -> Option<&SNP> {
        self.snps.iter().find(|snp| snp.rsid == rsid)
    }

    /// Get heterozygosity rate (proportion of heterozygous SNPs)
    pub fn heterozygosity_rate(&self) -> f64 {
        if self.snps.is_empty() {
            return 0.0;
        }

        let heterozygous_count = self.snps.iter().filter(|snp| snp.is_heterozygous()).count();
        heterozygous_count as f64 / self.snps.len() as f64
    }

    /// Get count of SNPs per chromosome
    pub fn chromosome_counts(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for snp in &self.snps {
            *counts.entry(snp.chromosome.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Get total number of SNPs
    pub fn total_snps(&self) -> usize {
        self.snps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_snp_from_line() {
        let line = "rs548049170\t1\t69869\tTT";
        let snp = SNP::from_line(line).unwrap();
        assert_eq!(snp.rsid, "rs548049170");
        assert_eq!(snp.chromosome, "1");
        assert_eq!(snp.position, 69869);
        assert_eq!(snp.genotype, "TT");
    }

    #[test]
    fn test_snp_from_line_x_chromosome() {
        let line = "rs123\tX\t12345\tAG";
        let snp = SNP::from_line(line).unwrap();
        assert_eq!(snp.chromosome, "X");
        assert_eq!(snp.position, 12345);
    }

    #[test]
    fn test_snp_from_line_invalid_format() {
        let line = "rs123\t1\t100";  // Missing genotype field
        let result = SNP::from_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_snp_from_line_invalid_position() {
        let line = "rs123\t1\tabc\tAA";  // Invalid position
        let result = SNP::from_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_snp_homozygous() {
        let snp = SNP::new("rs123".to_string(), "1".to_string(), 100, "AA".to_string());
        assert!(snp.is_homozygous());
        assert!(!snp.is_heterozygous());
    }

    #[test]
    fn test_snp_homozygous_all_bases() {
        let bases = vec!["AA", "TT", "GG", "CC"];
        for genotype in bases {
            let snp = SNP::new("rs123".to_string(), "1".to_string(), 100, genotype.to_string());
            assert!(snp.is_homozygous(), "Failed for genotype {}", genotype);
            assert!(!snp.is_heterozygous());
        }
    }

    #[test]
    fn test_snp_heterozygous() {
        let snp = SNP::new("rs123".to_string(), "1".to_string(), 100, "AG".to_string());
        assert!(snp.is_heterozygous());
        assert!(!snp.is_homozygous());
    }

    #[test]
    fn test_snp_heterozygous_all_combinations() {
        let combinations = vec!["AG", "AC", "AT", "GA", "GC", "GT", "CA", "CG", "CT", "TA", "TG", "TC"];
        for genotype in combinations {
            let snp = SNP::new("rs123".to_string(), "1".to_string(), 100, genotype.to_string());
            assert!(snp.is_heterozygous(), "Failed for genotype {}", genotype);
            assert!(!snp.is_homozygous());
        }
    }

    #[test]
    fn test_snp_invalid_genotype_length() {
        let snp = SNP::new("rs123".to_string(), "1".to_string(), 100, "A".to_string());
        assert!(!snp.is_homozygous());
        assert!(!snp.is_heterozygous());
    }

    #[test]
    fn test_genome_data_new() {
        let genome = GenomeData::new();
        assert_eq!(genome.snps.len(), 0);
        assert_eq!(genome.metadata.build, "GRCh37/hg19");
    }

    #[test]
    fn test_genome_data_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"# file_id: test-123
# signature: abc123
# timestamp: 2025-10-07 12:00:00
#
# rsid	chromosome	position	genotype
rs1	1	100	AA
rs2	1	200	AG
rs3	2	300	TT
rs4	X	400	GC
"#;
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let genome = GenomeData::from_file(temp_file.path()).unwrap();

        assert_eq!(genome.snps.len(), 4);
        assert_eq!(genome.metadata.file_id, Some("test-123".to_string()));
        assert_eq!(genome.metadata.signature, Some("abc123".to_string()));
        assert_eq!(genome.metadata.timestamp, Some("2025-10-07 12:00:00".to_string()));
    }

    #[test]
    fn test_genome_data_get_snps_by_chromosome() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()));
        genome.snps.push(SNP::new("rs3".to_string(), "2".to_string(), 300, "TT".to_string()));

        let chr1_snps = genome.get_snps_by_chromosome("1");
        assert_eq!(chr1_snps.len(), 2);

        let chr2_snps = genome.get_snps_by_chromosome("2");
        assert_eq!(chr2_snps.len(), 1);

        let chr3_snps = genome.get_snps_by_chromosome("3");
        assert_eq!(chr3_snps.len(), 0);
    }

    #[test]
    fn test_genome_data_find_snp() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()));

        let snp = genome.find_snp("rs1");
        assert!(snp.is_some());
        assert_eq!(snp.unwrap().position, 100);

        let not_found = genome.find_snp("rs999");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_genome_data_heterozygosity_rate() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string())); // Homozygous
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string())); // Heterozygous
        genome.snps.push(SNP::new("rs3".to_string(), "2".to_string(), 300, "TT".to_string())); // Homozygous
        genome.snps.push(SNP::new("rs4".to_string(), "2".to_string(), 400, "CT".to_string())); // Heterozygous

        let rate = genome.heterozygosity_rate();
        assert_eq!(rate, 0.5);
    }

    #[test]
    fn test_genome_data_heterozygosity_rate_empty() {
        let genome = GenomeData::new();
        let rate = genome.heterozygosity_rate();
        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_genome_data_chromosome_counts() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()));
        genome.snps.push(SNP::new("rs3".to_string(), "2".to_string(), 300, "TT".to_string()));
        genome.snps.push(SNP::new("rs4".to_string(), "X".to_string(), 400, "GC".to_string()));
        genome.snps.push(SNP::new("rs5".to_string(), "X".to_string(), 500, "AA".to_string()));

        let counts = genome.chromosome_counts();
        assert_eq!(*counts.get("1").unwrap(), 2);
        assert_eq!(*counts.get("2").unwrap(), 1);
        assert_eq!(*counts.get("X").unwrap(), 2);
    }

    #[test]
    fn test_genome_data_total_snps() {
        let mut genome = GenomeData::new();
        assert_eq!(genome.total_snps(), 0);

        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        assert_eq!(genome.total_snps(), 1);

        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()));
        assert_eq!(genome.total_snps(), 2);
    }

    #[test]
    fn test_genome_data_from_file_with_comments() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"# This is a comment
# Another comment
# rsid	chromosome	position	genotype
rs1	1	100	AA
# Comment in the middle
rs2	2	200	TT
"#;
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let genome = GenomeData::from_file(temp_file.path()).unwrap();
        assert_eq!(genome.snps.len(), 2);
    }

    #[test]
    fn test_genome_data_from_file_with_empty_lines() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"# rsid	chromosome	position	genotype
rs1	1	100	AA

rs2	2	200	TT

"#;
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let genome = GenomeData::from_file(temp_file.path()).unwrap();
        assert_eq!(genome.snps.len(), 2);
    }

    #[test]
    fn test_snp_equality() {
        let snp1 = SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string());
        let snp2 = SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string());
        let snp3 = SNP::new("rs2".to_string(), "1".to_string(), 100, "AA".to_string());

        assert_eq!(snp1, snp2);
        assert_ne!(snp1, snp3);
    }

    #[test]
    fn test_snp_clone() {
        let snp1 = SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string());
        let snp2 = snp1.clone();

        assert_eq!(snp1, snp2);
    }
}
