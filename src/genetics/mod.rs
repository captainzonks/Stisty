pub mod models;
pub mod analysis;

pub use models::{GenomeData, GenomeMetadata, SNP};
pub use analysis::{GenomeAnalyzer, GenomeSummary, lookup_trait_snps};