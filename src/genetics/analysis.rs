use super::models::{GenomeData, SNP};
use std::collections::HashMap;

/// Analyze genome data and generate summary statistics
pub struct GenomeAnalyzer<'a> {
    pub genome: &'a GenomeData,
}

impl<'a> GenomeAnalyzer<'a> {
    pub fn new(genome: &'a GenomeData) -> Self {
        Self { genome }
    }

    /// Get allele frequency for a specific position
    pub fn calculate_allele_frequencies(&self) -> HashMap<char, f64> {
        let mut allele_counts: HashMap<char, usize> = HashMap::new();
        let mut total_alleles = 0;

        for snp in &self.genome.snps {
            for allele in snp.genotype.chars() {
                if allele != '-' && allele != 'I' && allele != 'D' {
                    *allele_counts.entry(allele).or_insert(0) += 1;
                    total_alleles += 1;
                }
            }
        }

        allele_counts
            .into_iter()
            .map(|(allele, count)| (allele, count as f64 / total_alleles as f64))
            .collect()
    }

    /// Calculate transition/transversion ratio (Ts/Tv)
    /// Transitions: A<->G, C<->T
    /// Transversions: A<->C, A<->T, G<->C, G<->T
    pub fn transition_transversion_ratio(&self) -> f64 {
        let mut transitions = 0;
        let mut transversions = 0;

        for snp in &self.genome.snps {
            if snp.is_heterozygous() {
                let chars: Vec<char> = snp.genotype.chars().collect();
                let (a1, a2) = (chars[0], chars[1]);

                let is_transition = matches!(
                    (a1, a2),
                    ('A', 'G') | ('G', 'A') | ('C', 'T') | ('T', 'C')
                );

                if is_transition {
                    transitions += 1;
                } else {
                    transversions += 1;
                }
            }
        }

        if transversions == 0 {
            0.0
        } else {
            transitions as f64 / transversions as f64
        }
    }

    /// Generate a summary report of the genome data
    pub fn generate_summary(&self) -> GenomeSummary {
        let total_snps = self.genome.total_snps();
        let heterozygosity_rate = self.genome.heterozygosity_rate();
        let chromosome_counts = self.genome.chromosome_counts();
        let allele_frequencies = self.calculate_allele_frequencies();
        let ts_tv_ratio = self.transition_transversion_ratio();

        GenomeSummary {
            total_snps,
            heterozygosity_rate,
            chromosome_counts,
            allele_frequencies,
            ts_tv_ratio,
        }
    }
}

#[derive(Debug)]
pub struct GenomeSummary {
    pub total_snps: usize,
    pub heterozygosity_rate: f64,
    pub chromosome_counts: HashMap<String, usize>,
    pub allele_frequencies: HashMap<char, f64>,
    pub ts_tv_ratio: f64,
}

impl GenomeSummary {
    /// Display summary as formatted text
    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Genome Data Summary\n"));
        output.push_str(&format!("===================\n\n"));
        output.push_str(&format!("Total SNPs: {}\n", self.total_snps));
        output.push_str(&format!("Heterozygosity Rate: {:.4} ({:.2}%)\n",
            self.heterozygosity_rate, self.heterozygosity_rate * 100.0));
        output.push_str(&format!("Transition/Transversion Ratio: {:.4}\n\n", self.ts_tv_ratio));

        output.push_str("Allele Frequencies:\n");
        let mut alleles: Vec<_> = self.allele_frequencies.iter().collect();
        alleles.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        for (allele, freq) in alleles {
            output.push_str(&format!("  {}: {:.4} ({:.2}%)\n", allele, freq, freq * 100.0));
        }

        output.push_str("\nSNPs per Chromosome:\n");
        let mut chroms: Vec<_> = self.chromosome_counts.iter().collect();
        chroms.sort_by(|a, b| {
            // Sort numerically for 1-22, then X, Y, MT
            let a_num = a.0.parse::<u32>().ok();
            let b_num = b.0.parse::<u32>().ok();
            match (a_num, b_num) {
                (Some(a_n), Some(b_n)) => a_n.cmp(&b_n),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.0.cmp(b.0),
            }
        });
        for (chrom, count) in chroms {
            output.push_str(&format!("  Chr {}: {}\n", chrom, count));
        }

        output
    }
}

/// Find SNPs that match specific trait associations
/// This is a simple lookup - in practice, you'd want to use a database like dbSNP or ClinVar
pub fn lookup_trait_snps<'a>(genome: &'a GenomeData, rsids: &[&str]) -> Vec<&'a SNP> {
    rsids
        .iter()
        .filter_map(|&rsid| genome.find_snp(rsid))
        .collect()
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
            SNP::new("rs4".to_string(), "2".to_string(), 400, "CT".to_string()),
        ];
        genome
    }

    fn create_large_test_genome() -> GenomeData {
        let mut genome = GenomeData::new();
        // Create a more realistic dataset
        for i in 0..100 {
            let genotype = match i % 4 {
                0 => "AA",
                1 => "TT",
                2 => "GG",
                _ => "CC",
            };
            genome.snps.push(SNP::new(
                format!("rs{}", i),
                "1".to_string(),
                (i as u64 + 1) * 1000,
                genotype.to_string(),
            ));
        }
        // Add some heterozygous SNPs
        for i in 100..150 {
            let genotype = match i % 6 {
                0 => "AG",
                1 => "AT",
                2 => "AC",
                3 => "GT",
                4 => "GC",
                _ => "TC",
            };
            genome.snps.push(SNP::new(
                format!("rs{}", i),
                "2".to_string(),
                (i as u64 + 1) * 1000,
                genotype.to_string(),
            ));
        }
        genome
    }

    #[test]
    fn test_genome_analyzer_new() {
        let genome = create_test_genome();
        let analyzer = GenomeAnalyzer::new(&genome);
        assert_eq!(analyzer.genome.snps.len(), 4);
    }

    #[test]
    fn test_heterozygosity_rate() {
        let genome = create_test_genome();
        assert_eq!(genome.heterozygosity_rate(), 0.5);
    }

    #[test]
    fn test_allele_frequencies() {
        let genome = create_test_genome();
        let analyzer = GenomeAnalyzer::new(&genome);
        let freqs = analyzer.calculate_allele_frequencies();
        assert!(freqs.contains_key(&'A'));
        assert!(freqs.contains_key(&'T'));
        assert!(freqs.contains_key(&'G'));
        assert!(freqs.contains_key(&'C'));
    }

    #[test]
    fn test_allele_frequencies_exact() {
        let mut genome = GenomeData::new();
        // 3 A's (2 from AA, 1 from AT), 1 T (from AT)
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "AT".to_string()));

        let analyzer = GenomeAnalyzer::new(&genome);
        let freqs = analyzer.calculate_allele_frequencies();

        // Total: 4 alleles (3 A, 1 T)
        assert_eq!(freqs.len(), 2);
        assert!((freqs[&'A'] - 3.0 / 4.0).abs() < 0.001);
        assert!((freqs[&'T'] - 1.0 / 4.0).abs() < 0.001);
    }

    #[test]
    fn test_allele_frequencies_empty_genome() {
        let genome = GenomeData::new();
        let analyzer = GenomeAnalyzer::new(&genome);
        let freqs = analyzer.calculate_allele_frequencies();
        assert!(freqs.is_empty());
    }

    #[test]
    fn test_transition_transversion_ratio() {
        let mut genome = GenomeData::new();
        // Transitions: AG, GA, CT, TC
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AG".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "GA".to_string()));
        genome.snps.push(SNP::new("rs3".to_string(), "1".to_string(), 300, "CT".to_string()));
        genome.snps.push(SNP::new("rs4".to_string(), "1".to_string(), 400, "TC".to_string()));

        // Transversions: AC, AT, GC, GT
        genome.snps.push(SNP::new("rs5".to_string(), "2".to_string(), 500, "AC".to_string()));
        genome.snps.push(SNP::new("rs6".to_string(), "2".to_string(), 600, "AT".to_string()));

        let analyzer = GenomeAnalyzer::new(&genome);
        let ratio = analyzer.transition_transversion_ratio();

        // 4 transitions, 2 transversions = 2.0
        assert!((ratio - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_transition_transversion_ratio_no_transversions() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AG".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "CT".to_string()));

        let analyzer = GenomeAnalyzer::new(&genome);
        let ratio = analyzer.transition_transversion_ratio();
        assert_eq!(ratio, 0.0); // Should handle division by zero
    }

    #[test]
    fn test_transition_transversion_ratio_only_homozygous() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "TT".to_string()));

        let analyzer = GenomeAnalyzer::new(&genome);
        let ratio = analyzer.transition_transversion_ratio();
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn test_generate_summary() {
        let genome = create_test_genome();
        let analyzer = GenomeAnalyzer::new(&genome);
        let summary = analyzer.generate_summary();

        assert_eq!(summary.total_snps, 4);
        assert_eq!(summary.heterozygosity_rate, 0.5);
        assert!(summary.chromosome_counts.contains_key("1"));
        assert!(summary.chromosome_counts.contains_key("2"));
        assert!(!summary.allele_frequencies.is_empty());
    }

    #[test]
    fn test_generate_summary_large_genome() {
        let genome = create_large_test_genome();
        let analyzer = GenomeAnalyzer::new(&genome);
        let summary = analyzer.generate_summary();

        assert_eq!(summary.total_snps, 150);
        assert!(summary.heterozygosity_rate > 0.0);
        assert!(summary.ts_tv_ratio >= 0.0);
    }

    #[test]
    fn test_summary_display() {
        let genome = create_test_genome();
        let analyzer = GenomeAnalyzer::new(&genome);
        let summary = analyzer.generate_summary();

        let display = summary.display();
        assert!(display.contains("Genome Data Summary"));
        assert!(display.contains("Total SNPs:"));
        assert!(display.contains("Heterozygosity Rate:"));
        assert!(display.contains("Allele Frequencies:"));
        assert!(display.contains("SNPs per Chromosome:"));
    }

    #[test]
    fn test_lookup_trait_snps() {
        let genome = create_test_genome();
        let results = lookup_trait_snps(&genome, &["rs1", "rs3"]);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rsid, "rs1");
        assert_eq!(results[1].rsid, "rs3");
    }

    #[test]
    fn test_lookup_trait_snps_not_found() {
        let genome = create_test_genome();
        let results = lookup_trait_snps(&genome, &["rs999", "rs888"]);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_lookup_trait_snps_partial_match() {
        let genome = create_test_genome();
        let results = lookup_trait_snps(&genome, &["rs1", "rs999", "rs3"]);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rsid, "rs1");
        assert_eq!(results[1].rsid, "rs3");
    }

    #[test]
    fn test_lookup_trait_snps_empty_list() {
        let genome = create_test_genome();
        let results = lookup_trait_snps(&genome, &[]);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_allele_frequencies_ignores_special_chars() {
        let mut genome = GenomeData::new();
        genome.snps.push(SNP::new("rs1".to_string(), "1".to_string(), 100, "AA".to_string()));
        genome.snps.push(SNP::new("rs2".to_string(), "1".to_string(), 200, "--".to_string())); // Should be ignored
        genome.snps.push(SNP::new("rs3".to_string(), "1".to_string(), 300, "II".to_string())); // Should be ignored
        genome.snps.push(SNP::new("rs4".to_string(), "1".to_string(), 400, "DD".to_string())); // Should be ignored

        let analyzer = GenomeAnalyzer::new(&genome);
        let freqs = analyzer.calculate_allele_frequencies();

        // Only A's should be counted
        assert_eq!(freqs.len(), 1);
        assert!(freqs.contains_key(&'A'));
    }

    #[test]
    fn test_chromosome_counts_from_summary() {
        let mut genome = GenomeData::new();
        for i in 0..10 {
            genome.snps.push(SNP::new(format!("rs{}", i), "1".to_string(), (i + 1) * 100, "AA".to_string()));
        }
        for i in 10..15 {
            genome.snps.push(SNP::new(format!("rs{}", i), "X".to_string(), (i + 1) * 100, "TT".to_string()));
        }

        let analyzer = GenomeAnalyzer::new(&genome);
        let summary = analyzer.generate_summary();

        assert_eq!(*summary.chromosome_counts.get("1").unwrap(), 10);
        assert_eq!(*summary.chromosome_counts.get("X").unwrap(), 5);
    }

    #[test]
    fn test_realistic_ts_tv_ratio() {
        let mut genome = GenomeData::new();

        // Simulate realistic human genome Ts/Tv ratio (~2.0-2.1)
        // Add 20 transitions
        for i in 0..10 {
            genome.snps.push(SNP::new(format!("rs{}", i), "1".to_string(), (i + 1) * 100, "AG".to_string()));
        }
        for i in 10..20 {
            genome.snps.push(SNP::new(format!("rs{}", i), "1".to_string(), (i + 1) * 100, "CT".to_string()));
        }

        // Add 10 transversions
        for i in 20..25 {
            genome.snps.push(SNP::new(format!("rs{}", i), "2".to_string(), (i + 1) * 100, "AT".to_string()));
        }
        for i in 25..30 {
            genome.snps.push(SNP::new(format!("rs{}", i), "2".to_string(), (i + 1) * 100, "GC".to_string()));
        }

        let analyzer = GenomeAnalyzer::new(&genome);
        let ratio = analyzer.transition_transversion_ratio();

        // Should be 20/10 = 2.0
        assert!((ratio - 2.0).abs() < 0.001);
    }
}