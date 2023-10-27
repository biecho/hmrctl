// Importing submodule definitions
pub mod strategy;
pub mod pagemap_strategy;
pub mod kvm_hypercall_strategy;
pub mod error;

// Re-exporting public-facing components for external modules
pub use strategy::TranslationStrategy;
pub use error::V2PError;

// You can also define common types or utilities here.
// For example, you might have a common type for VirtualAddress and PhysicalAddress
pub type VirtualAddress = u64;
pub type PhysicalAddress = u64;
