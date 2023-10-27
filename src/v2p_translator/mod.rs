// Importing submodule definitions
pub mod pagemap_translator;
pub mod kvm_hypercall_translator;
pub mod error;
pub mod translator;

// Re-exporting public-facing components for external modules
pub use error::V2PError;

// You can also define common types or utilities here.
// For example, you might have a common type for VirtualAddress and PhysicalAddress
pub type VirtualAddress = u64;
pub type PhysicalAddress = u64;
