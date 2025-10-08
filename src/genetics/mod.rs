pub mod models;
pub mod analysis;
pub mod vcf;

pub use models::{GenomeData, GenomeMetadata, SNP};
pub use analysis::{GenomeAnalyzer, GenomeSummary, lookup_trait_snps};
pub use vcf::VcfGenerator;