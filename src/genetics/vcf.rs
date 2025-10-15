use super::models::{GenomeData, SNP};
use super::reference::{ReferenceDatabase, SnpReference};
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;

#[cfg(feature = "cli")]
use flate2::write::GzEncoder;
#[cfg(feature = "cli")]
use flate2::Compression;
#[cfg(feature = "cli")]
use std::io::Write;

/// VCF (Variant Call Format) generator for genome data
pub struct VcfGenerator<'a> {
    genome: &'a GenomeData,
    /// Optional reference database for proper REF/ALT alleles
    reference_db: Option<&'a ReferenceDatabase>,
    /// Index for fast reference lookups
    reference_index: Option<&'a HashMap<String, usize>>,
}

impl<'a> VcfGenerator<'a> {
    pub fn new(genome: &'a GenomeData) -> Self {
        Self {
            genome,
            reference_db: None,
            reference_index: None,
        }
    }

    /// Create a new VCF generator with reference database support
    pub fn with_reference(
        genome: &'a GenomeData,
        reference_db: &'a ReferenceDatabase,
        reference_index: &'a HashMap<String, usize>,
    ) -> Self {
        Self {
            genome,
            reference_db: Some(reference_db),
            reference_index: Some(reference_index),
        }
    }

    /// Generate VCF file content for a specific chromosome or all chromosomes
    ///
    /// # Arguments
    /// * `chromosome` - Optional chromosome filter (e.g., "1", "X"). If None, includes all chromosomes.
    ///
    /// # Returns
    /// String containing the complete VCF file content
    pub fn generate_vcf(&self, chromosome: Option<&str>) -> Result<String> {
        let mut output = String::new();

        // Write VCF header
        self.write_header(&mut output)?;

        // Get SNPs to export (filtered by chromosome if specified)
        let snps: Vec<&SNP> = match chromosome {
            Some(chr) => self.genome.get_snps_by_chromosome(chr),
            None => self.genome.snps.iter().collect(),
        };

        // Sort SNPs by chromosome and position
        let mut sorted_snps = snps.clone();
        sorted_snps.sort_by(|a, b| {
            // First sort by chromosome (numerically if possible, then alphabetically)
            let chr_cmp = compare_chromosomes(&a.chromosome, &b.chromosome);
            if chr_cmp != std::cmp::Ordering::Equal {
                return chr_cmp;
            }
            // Then sort by position
            a.position.cmp(&b.position)
        });

        // Write VCF data lines
        for snp in sorted_snps {
            self.write_variant_line(&mut output, snp)?;
        }

        Ok(output)
    }

    /// Generate multiple VCF files for chromosomes 1-22 (autosomes only)
    ///
    /// Returns a HashMap mapping chromosome names to VCF content
    /// Suitable for Michigan Imputation Server which requires separate files per chromosome
    ///
    /// # Returns
    /// HashMap where keys are chromosome names ("1" through "22") and values are VCF content
    pub fn generate_batch_vcf(&self) -> Result<HashMap<String, String>> {
        let mut vcf_files = HashMap::new();

        // Michigan Imputation Server requires autosomes only (chromosomes 1-22)
        for chr_num in 1..=22 {
            let chr = chr_num.to_string();
            let vcf_content = self.generate_vcf(Some(&chr))?;

            // Only include if there are actual variants (not just header)
            let has_data = vcf_content.lines().any(|line| !line.starts_with('#'));
            if has_data {
                vcf_files.insert(chr, vcf_content);
            }
        }

        Ok(vcf_files)
    }

    /// Compress VCF content to bgzip/gzip format (CLI feature only)
    ///
    /// Michigan Imputation Server requires bgzip-compressed files (.vcf.gz)
    /// This uses gzip compression which is compatible with bgzip format
    #[cfg(feature = "cli")]
    pub fn compress_vcf(vcf_content: &str) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(vcf_content.as_bytes())?;
        Ok(encoder.finish()?)
    }

    /// Generate and compress multiple VCF files for chromosomes 1-22 (CLI feature only)
    ///
    /// Returns a HashMap mapping chromosome names to compressed VCF data
    /// Ready for Michigan Imputation Server upload
    ///
    /// # Returns
    /// HashMap where keys are chromosome names ("1" through "22") and values are gzipped VCF data
    #[cfg(feature = "cli")]
    pub fn generate_batch_vcf_compressed(&self) -> Result<HashMap<String, Vec<u8>>> {
        let vcf_files = self.generate_batch_vcf()?;
        let mut compressed_files = HashMap::new();

        for (chr, vcf_content) in vcf_files {
            let compressed = Self::compress_vcf(&vcf_content)?;
            compressed_files.insert(chr, compressed);
        }

        Ok(compressed_files)
    }

    /// Write VCF header lines
    fn write_header(&self, output: &mut String) -> Result<()> {
        // File format version
        output.push_str("##fileformat=VCFv4.2\n");

        // File date
        let now = Utc::now();
        output.push_str(&format!("##fileDate={}\n", now.format("%Y%m%d")));

        // Source information
        output.push_str("##source=Stisty-23andMe-Converter\n");

        // Add appropriate source note based on whether we have a reference database
        if self.reference_db.is_some() {
            output.push_str("##sourceNote=Converted from 23andMe raw data. Genotypes are UNPHASED. REF/ALT alleles from GRCh37 reference genome. Only biallelic SNPs with known REF/ALT included.\n");
        } else {
            output.push_str("##sourceNote=Converted from 23andMe raw data. Genotypes are UNPHASED. REF/ALT alleles inferred from genotype (not reference genome). Not suitable for imputation.\n");
        }

        // Reference genome
        let reference = &self.genome.metadata.build;
        output.push_str(&format!("##reference={}\n", reference));

        // Contig information for each chromosome present in the data
        let mut chromosomes: Vec<String> = self.genome.chromosome_counts()
            .keys()
            .cloned()
            .collect();
        chromosomes.sort_by(|a, b| compare_chromosomes(a, b));

        for chr in chromosomes {
            output.push_str(&format!("##contig=<ID={}>\n", chr));
        }

        // INFO field definitions
        output.push_str("##INFO=<ID=NS,Number=1,Type=Integer,Description=\"Number of samples with data\">\n");
        output.push_str("##INFO=<ID=PHASED,Number=0,Type=Flag,Description=\"Indicates if genotype is phased\">\n");

        // FORMAT field definitions
        output.push_str("##FORMAT=<ID=GT,Number=1,Type=String,Description=\"Genotype\">\n");

        // FILTER definitions
        output.push_str("##FILTER=<ID=PASS,Description=\"All filters passed\">\n");

        // Column header line
        // Michigan Imputation Server requires at least 5 samples
        // We include 5 anonymous samples (samp1-5) + user's sample (samp51) = 6 total
        output.push_str("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsamp1\tsamp2\tsamp3\tsamp4\tsamp5\tsamp51\n");

        Ok(())
    }

    /// Write a single variant line to the VCF
    fn write_variant_line(&self, output: &mut String, snp: &SNP) -> Result<()> {
        // Look up reference information if available
        let ref_info = if let (Some(db), Some(index)) = (self.reference_db, self.reference_index) {
            db.lookup(&snp.rsid, index)
        } else {
            None
        };

        // IMPORTANT: Only include SNPs that are in the reference panel
        // This matches the R script behavior: merge(x, y, all.x=TRUE, all.y=FALSE)
        // We skip SNPs from user data that aren't in the reference panel to avoid
        // arbitrary REF/ALT assignments that don't match the imputation server's reference
        if ref_info.is_none() {
            return Ok(());
        }

        // Parse genotype to determine REF and ALT alleles
        let (ref_allele, alt_allele, genotype_string) = self.parse_genotype(&snp.genotype, ref_info.as_ref())?;

        // Skip if we can't determine alleles (e.g., for deletions, insertions, invalid genotypes, or missing ALT)
        // Michigan Imputation Server requires both REF and ALT to be defined
        if ref_allele == "." || alt_allele == "." || genotype_string == "./." {
            return Ok(());
        }

        // Note: 23andMe provides unphased genotypes (no haplotype information)

        // CHROM
        output.push_str(&snp.chromosome);
        output.push('\t');

        // POS
        output.push_str(&snp.position.to_string());
        output.push('\t');

        // ID (rsid)
        output.push_str(&snp.rsid);
        output.push('\t');

        // REF
        output.push_str(&ref_allele);
        output.push('\t');

        // ALT
        output.push_str(&alt_allele);
        output.push('\t');

        // QUAL (unknown for 23andMe data)
        output.push('.');
        output.push('\t');

        // FILTER (PASS by default for 23andMe data)
        output.push_str("PASS");
        output.push('\t');

        // INFO
        output.push_str("NS=6");
        output.push('\t');

        // FORMAT
        output.push_str("GT");
        output.push('\t');

        // Sample genotypes: 5 anonymous samples (samp1-5) + user's genotype (samp51)
        // Use real genotypes from anonymous samples if available
        if let Some(snp_ref) = &ref_info {
            for i in 0..5 {
                output.push_str(&snp_ref.sample_genotypes[i]);
                output.push('\t');
            }
        } else {
            // Fallback to 0/0 if no reference data (shouldn't happen with reference database)
            output.push_str("0/0\t0/0\t0/0\t0/0\t0/0\t");
        }
        output.push_str(&genotype_string);
        output.push('\n');

        Ok(())
    }

    /// Parse 23andMe genotype into VCF format with reference database support
    ///
    /// Returns (REF, ALT, genotype_string)
    ///
    /// With reference database:
    /// - REF allele comes from the reference genome
    /// - ALT allele is the non-reference variant
    /// - Genotype (GT) is properly encoded as 0/0, 0/1, or 1/1
    ///
    /// Without reference database (fallback):
    /// - For heterozygous: first allele is REF, second is ALT
    /// - For homozygous: the allele is REF, no ALT
    fn parse_genotype(&self, genotype: &str, ref_info: Option<&SnpReference>) -> Result<(String, String, String)> {
        if genotype.len() != 2 {
            // Invalid genotype or deletion/insertion
            return Ok((".".to_string(), ".".to_string(), "./.".to_string()));
        }

        let chars: Vec<char> = genotype.chars().collect();
        let allele1 = chars[0];
        let allele2 = chars[1];

        // Handle special characters (deletions, insertions, no-calls)
        if allele1 == '-' || allele2 == '-' ||
           allele1 == 'I' || allele2 == 'I' ||
           allele1 == 'D' || allele2 == 'D' {
            return Ok((".".to_string(), ".".to_string(), "./.".to_string()));
        }

        // Validate that both alleles are valid nucleotides
        if !is_valid_nucleotide(allele1) || !is_valid_nucleotide(allele2) {
            return Ok((".".to_string(), ".".to_string(), "./.".to_string()));
        }

        // If we have reference information, use it for proper REF/ALT
        if let Some(ref_data) = ref_info {
            let ref_allele = ref_data.ref_allele;
            let alt_allele = ref_data.alt_allele;

            // Skip SNPs where we don't have a valid ALT allele in the reference database
            // This matches the behavior of imputation preparation tools which require
            // both REF and ALT alleles to be defined
            if alt_allele == 'N' || ref_allele == 'N' {
                return Ok((".".to_string(), ".".to_string(), "./.".to_string()));
            }

            // Determine genotype by counting ALT alleles (matches R script logic)
            // g1 = (allele1 == ALT) ? 1 : 0
            // g2 = (allele2 == ALT) ? 1 : 0
            // genotype = "g1/g2"
            let g1 = if allele1 == alt_allele { "1" } else { "0" };
            let g2 = if allele2 == alt_allele { "1" } else { "0" };
            let gt = format!("{}/{}", g1, g2);

            return Ok((
                ref_allele.to_string(),
                alt_allele.to_string(),
                gt.to_string(),
            ));
        }

        // Fallback: No reference database available
        // Use the old logic (not suitable for imputation servers)
        if allele1 == allele2 {
            // Homozygous: both alleles are the same
            // In VCF, we represent this as REF with no ALT (or ALT = ".")
            // Genotype is 0/0 (both alleles match reference)
            Ok((allele1.to_string(), ".".to_string(), "0/0".to_string()))
        } else {
            // Heterozygous: two different alleles
            // We'll treat the first allele as REF and second as ALT
            // Genotype is 0/1 (one REF, one ALT)
            Ok((allele1.to_string(), allele2.to_string(), "0/1".to_string()))
        }
    }
}

/// Compare chromosome identifiers for sorting
/// Numeric chromosomes (1-22) sort numerically, followed by X, Y, MT
fn compare_chromosomes(a: &str, b: &str) -> std::cmp::Ordering {
    let a_num = a.parse::<u32>().ok();
    let b_num = b.parse::<u32>().ok();

    match (a_num, b_num) {
        (Some(a_n), Some(b_n)) => a_n.cmp(&b_n),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => {
            // Special ordering for X, Y, MT using match for performance
            fn chrom_order(s: &str) -> u32 {
                match s {
                    "X" => 0,
                    "Y" => 1,
                    "MT" | "M" => 2,
                    _ => 99,
                }
            }

            let a_order = chrom_order(a);
            let b_order = chrom_order(b);
            a_order.cmp(&b_order).then_with(|| a.cmp(b))
        }
    }
}

/// Check if a character is a valid nucleotide
fn is_valid_nucleotide(c: char) -> bool {
    matches!(c, 'A' | 'T' | 'G' | 'C')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetics::models::{GenomeData, SNP};

    fn create_test_genome() -> GenomeData {
        let mut genome = GenomeData::new();
        genome.snps = vec![
            SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()),
            SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()),
            SNP::new("rs3".to_string(), "2".to_string(), 300, "TT".to_string()),
            SNP::new("rs4".to_string(), "X".to_string(), 400, "CT".to_string()),
        ];
        genome
    }

    #[test]
    fn test_vcf_generator_new() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);
        assert_eq!(generator.genome.snps.len(), 4);
    }

    #[test]
    fn test_parse_genotype_homozygous() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("AA", None).unwrap();
        assert_eq!(ref_allele, "A");
        assert_eq!(alt_allele, ".");  // No ALT allele for homozygous without reference
        assert_eq!(gt, "0/0");
    }

    #[test]
    fn test_parse_genotype_heterozygous() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("AG", None).unwrap();
        assert_eq!(ref_allele, "A");
        assert_eq!(alt_allele, "G");
        assert_eq!(gt, "0/1");
    }

    #[test]
    fn test_parse_genotype_invalid() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("--", None).unwrap();
        assert_eq!(ref_allele, ".");  // Invalid genotypes return missing
        assert_eq!(alt_allele, ".");
        assert_eq!(gt, "./.");
    }

    #[test]
    fn test_parse_genotype_deletion() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("DD", None).unwrap();
        assert_eq!(ref_allele, ".");  // Deletions return missing
        assert_eq!(alt_allele, ".");
        assert_eq!(gt, "./.");
    }

    #[test]
    fn test_generate_vcf_all_chromosomes() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);
        let vcf = generator.generate_vcf(None).unwrap();

        // Check header
        assert!(vcf.contains("##fileformat=VCFv4.2"));
        assert!(vcf.contains("##source=Stisty-23andMe-Converter"));
        // New format includes 5 samples for Michigan Imputation Server compatibility
        assert!(vcf.contains("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsamp1\tsamp2\tsamp3\tsamp4\tsamp5"));

        // Note: Without reference database, homozygous variants have ALT="." and are skipped
        // Only heterozygous variants (rs2, rs4) will be included
        assert!(vcf.contains("rs2"));  // Heterozygous AG
        assert!(vcf.contains("rs4"));  // Heterozygous CT
    }

    #[test]
    fn test_generate_vcf_specific_chromosome() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);
        let vcf = generator.generate_vcf(Some("1")).unwrap();

        // Should include chromosome 1 heterozygous variant (rs1 is homozygous and skipped)
        assert!(vcf.contains("rs2"));  // Heterozygous AG

        // Should not include other chromosomes
        assert!(!vcf.contains("rs3"));
        assert!(!vcf.contains("rs4"));
    }

    #[test]
    fn test_compare_chromosomes() {
        assert_eq!(compare_chromosomes("1", "2"), std::cmp::Ordering::Less);
        assert_eq!(compare_chromosomes("10", "2"), std::cmp::Ordering::Greater);
        assert_eq!(compare_chromosomes("22", "X"), std::cmp::Ordering::Less);
        assert_eq!(compare_chromosomes("X", "Y"), std::cmp::Ordering::Less);
        assert_eq!(compare_chromosomes("Y", "MT"), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_is_valid_nucleotide() {
        assert!(is_valid_nucleotide('A'));
        assert!(is_valid_nucleotide('T'));
        assert!(is_valid_nucleotide('G'));
        assert!(is_valid_nucleotide('C'));
        assert!(!is_valid_nucleotide('-'));
        assert!(!is_valid_nucleotide('I'));
        assert!(!is_valid_nucleotide('D'));
        assert!(!is_valid_nucleotide('N'));
    }

    #[test]
    fn test_vcf_format_structure() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);
        let vcf = generator.generate_vcf(None).unwrap();

        let lines: Vec<&str> = vcf.lines().collect();

        // Find the first data line (after headers)
        let data_line = lines.iter()
            .find(|line| !line.starts_with('#'))
            .unwrap();

        // Check that the line has the correct number of fields
        // New format: 9 fixed fields + 5 samples = 14 total
        let fields: Vec<&str> = data_line.split('\t').collect();
        assert_eq!(fields.len(), 14); // CHROM, POS, ID, REF, ALT, QUAL, FILTER, INFO, FORMAT, samp1, samp2, samp3, samp4, samp5
    }

    #[test]
    fn test_vcf_sorting_by_position() {
        let mut genome = GenomeData::new();
        // Use all heterozygous genotypes so they won't be filtered out
        genome.snps.push(SNP::new("rs3".to_string(), "1".to_string(), 300, "GT".to_string()));
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AC".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AG".to_string()));

        let generator = VcfGenerator::new(&genome);
        let vcf = generator.generate_vcf(None).unwrap();

        // Extract data lines
        let data_lines: Vec<&str> = vcf.lines()
            .filter(|line| !line.starts_with('#'))
            .collect();

        // Check that positions are in ascending order
        let positions: Vec<u64> = data_lines.iter()
            .map(|line| {
                let fields: Vec<&str> = line.split('\t').collect();
                fields[1].parse::<u64>().unwrap()
            })
            .collect();

        assert_eq!(positions, vec![100, 200, 300]);
    }
}
