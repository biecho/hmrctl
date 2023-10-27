// src/config/models.rs

use serde::Deserialize;
use std::fmt;

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

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory Config:\n  {}", self.memory)?;
        writeln!(f, "Physical Translation:\n  {}", self.physical_translation)?;
        writeln!(f, "DRAM Config:\n  {}", self.dram)?;
        write!(f, "Hammering Config:\n  {}", self.hammering)
    }
}

impl fmt::Display for MemoryConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.allocation)
    }
}

impl fmt::Display for AllocationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hugepages Mount: {}\n  Size (bytes): {}", self.hugepages_mount, self.size_bytes)
    }
}

impl fmt::Display for PhysicalTranslation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Method: {}", self.method)
    }
}

impl fmt::Display for TranslationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationMethod::Selfmap => write!(f, "Selfmap"),
            TranslationMethod::Hypercall => write!(f, "Hypercall"),
        }
    }
}

impl fmt::Display for DramConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}

impl fmt::Display for DramLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DramLayout {{")?;
        writeln!(f, "  addressing_functions: [")?;
        for func in &self.addressing_functions {
            writeln!(f, "    0x{:x},", func)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  row_mask: 0x{:x},", self.row_mask)?;
        writeln!(f, "  col_mask: 0x{:x}", self.col_mask)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for HammeringConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rounds: {}\n  Base Offset: {}\n  Pattern Length: {}\n  Hammer Data: {}\n  Memory Access Strategy: {}",
            self.rounds, self.base_offset, self.pattern_length, self.hammer_data, self.memory_access_strategy
        )
    }
}

impl fmt::Display for HammerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HammerData::Random => write!(f, "Random"),
            // Add more when other strategies are available...
        }
    }
}

impl fmt::Display for MemoryAccessStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryAccessStrategy::Native => write!(f, "Native"),
            MemoryAccessStrategy::HypervisorAssistance { percentage } => {
                write!(f, "Hypervisor Assistance (Percentage: {}%)", percentage)
            },
        }
    }
}

