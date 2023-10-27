use crate::config::models::PhysicalTranslationStrategy;
use crate::v2p_translator::pagemap_strategy::PagemapStrategy;

pub enum StrategyInstance {
    Pagemap(PagemapStrategy),
    Hypercall, // Assume you'll have a separate struct for this
    // ... add other strategies as needed ...
}

impl StrategyInstance {
    pub fn from_config(config: PhysicalTranslationStrategy) -> Self {
        match config {
            PhysicalTranslationStrategy::Selfmap(sm_config) => {
                StrategyInstance::Pagemap(PagemapStrategy::new(sm_config))
            }
            _ => StrategyInstance::Hypercall
        }
    }
}
