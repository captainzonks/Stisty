use super::models::{GenomeData, SNP};
use anyhow::Result;
use chrono::Utc;

/// VCF (Variant Call Format) generator for genome data
pub struct VcfGenerator<'a> {
    genome: &'a GenomeData,
}

impl<'a> VcfGenerator<'a> {
    pub fn new(genome: &'a GenomeData) -> Self {
        Self { genome }
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

    /// Write VCF header lines
    fn write_header(&self, output: &mut String) -> Result<()> {
        // File format version
        output.push_str("##fileformat=VCFv4.2\n");

        // File date
        let now = Utc::now();
        output.push_str(&format!("##fileDate={}\n", now.format("%Y%m%d")));

        // Source information
        output.push_str("##source=Stisty-23andMe-Converter\n");

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

        // FORMAT field definitions
        output.push_str("##FORMAT=<ID=GT,Number=1,Type=String,Description=\"Genotype\">\n");

        // FILTER definitions
        output.push_str("##FILTER=<ID=PASS,Description=\"All filters passed\">\n");

        // Column header line
        output.push_str("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tSAMPLE\n");

        Ok(())
    }

    /// Write a single variant line to the VCF
    fn write_variant_line(&self, output: &mut String, snp: &SNP) -> Result<()> {
        // Parse genotype to determine REF and ALT alleles
        let (ref_allele, alt_allele, genotype_string) = self.parse_genotype(&snp.genotype)?;

        // Skip if we can't determine alleles (e.g., for deletions, insertions, or invalid genotypes)
        if ref_allele == "." || genotype_string == "./." {
            return Ok(());
        }

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
        output.push_str("NS=1");
        output.push('\t');

        // FORMAT
        output.push_str("GT");
        output.push('\t');

        // Sample genotype
        output.push_str(&genotype_string);
        output.push('\n');

        Ok(())
    }

    /// Parse 23andMe genotype into VCF format
    ///
    /// Returns (REF, ALT, genotype_string)
    ///
    /// Note: 23andMe data doesn't include reference allele information,
    /// so we make reasonable assumptions:
    /// - For heterozygous genotypes: first allele is REF, second is ALT
    /// - For homozygous genotypes: the allele is REF, no ALT
    fn parse_genotype(&self, genotype: &str) -> Result<(String, String, String)> {
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

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("AA").unwrap();
        assert_eq!(ref_allele, "A");
        assert_eq!(alt_allele, ".");
        assert_eq!(gt, "0/0");
    }

    #[test]
    fn test_parse_genotype_heterozygous() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("AG").unwrap();
        assert_eq!(ref_allele, "A");
        assert_eq!(alt_allele, "G");
        assert_eq!(gt, "0/1");
    }

    #[test]
    fn test_parse_genotype_invalid() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("--").unwrap();
        assert_eq!(ref_allele, ".");
        assert_eq!(alt_allele, ".");
        assert_eq!(gt, "./.");
    }

    #[test]
    fn test_parse_genotype_deletion() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);

        let (ref_allele, alt_allele, gt) = generator.parse_genotype("DD").unwrap();
        assert_eq!(ref_allele, ".");
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
        assert!(vcf.contains("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tSAMPLE"));

        // Check that variants are present
        assert!(vcf.contains("rs1"));
        assert!(vcf.contains("rs2"));
        assert!(vcf.contains("rs3"));
        assert!(vcf.contains("rs4"));
    }

    #[test]
    fn test_generate_vcf_specific_chromosome() {
        let genome = create_test_genome();
        let generator = VcfGenerator::new(&genome);
        let vcf = generator.generate_vcf(Some("1")).unwrap();

        // Should include chromosome 1 variants
        assert!(vcf.contains("rs1"));
        assert!(vcf.contains("rs2"));

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
        let fields: Vec<&str> = data_line.split('\t').collect();
        assert_eq!(fields.len(), 10); // CHROM, POS, ID, REF, ALT, QUAL, FILTER, INFO, FORMAT, SAMPLE
    }

    #[test]
    fn test_vcf_sorting_by_position() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs3".to_string(), "1".to_string(), 300, "TT".to_string()));
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
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
