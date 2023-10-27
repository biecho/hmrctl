// src/config/models.rs

use serde::Deserialize;

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub memory: MemoryConfig,
    pub physical_translation: PhysicalTranslation,
    pub dram: DramConfig,
    pub hammering: HammeringConfig,
}

/// Configuration related to memory allocation.
#[derive(Debug, Deserialize)]
pub struct MemoryConfig {
    pub allocation: AllocationConfig,
}

/// Detailed allocation settings.
#[derive(Debug, Deserialize)]
pub struct AllocationConfig {
    pub hugepages_mount: String,
    pub size_bytes: usize,
}

/// Strategy for translating virtual to physical addresses.
#[derive(Debug, Deserialize)]
pub struct PhysicalTranslation {
    pub method: TranslationMethod,
}

/// Different methods for physical address translation.
#[derive(Debug, Deserialize)]
pub enum TranslationMethod {
    Selfmap,
    Hypercall,
}

/// Configuration specific to the DRAM module and its layout.
#[derive(Debug, Deserialize)]
pub struct DramConfig {
    pub layout: DramLayout,
}

/// DRAM addressing layout configuration.
#[derive(Debug, Deserialize)]
pub struct DramLayout {
    pub addressing_functions: Vec<u64>,
    pub row_mask: u64,
    pub col_mask: u64,
}

/// Configuration related to the hammering process.
#[derive(Debug, Deserialize)]
pub struct HammeringConfig {
    pub rounds: u32,
    pub base_offset: usize,
    pub pattern_length: usize,
    pub hammer_data: HammerData,
    pub memory_access_strategy: MemoryAccessStrategy,
}

/// Different strategies for generating data to be hammered.
#[derive(Debug, Deserialize)]
pub enum HammerData {
    Random,
    // Potentially other data generation strategies...
}

/// Memory access strategy during hammering.
#[derive(Debug, Deserialize)]
#[serde(tag = "strategy")]
pub enum MemoryAccessStrategy {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "hypervisor")]
    HypervisorAssistance {
        percentage: u8,
        // You might add other configuration parameters related to the hypervisor here.
    },
}
