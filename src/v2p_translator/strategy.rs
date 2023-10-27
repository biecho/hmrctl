use super::{V2PError, VirtualAddress};
use super::PhysicalAddress;

pub enum TranslationStrategy {
    // Pagemap(PagemapStrategy),
    // Hypervisor(HypervisorStrategy),
    // ... other strategies can be added in the future
}

impl TranslationStrategy {
    pub fn translate_to_physical(&self, virtual_address: VirtualAddress)
                                 -> Result<PhysicalAddress, V2PError> {
        Ok(0)
        // match self {
        //     TranslationStrategy::Pagemap(strategy) =>
        //         strategy.translate_to_physical(virtual_address),
        //     TranslationStrategy::Hypervisor(strategy) =>
        //         strategy.translate_to_physical(virtual_address),
        // }
    }

}
