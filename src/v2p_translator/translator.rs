use crate::config::models::PhysicalTranslationStrategy;
use crate::v2p_translator::kvm_hypercall_translator::KvmHypercallStrategy;
use crate::v2p_translator::pagemap_translator::PagemapStrategy;
use crate::v2p_translator::V2PError;

pub trait VirtualToPhysicalTranslator {
    fn translate_to_physical(&self, virtual_address: u64) -> Result<u64, V2PError>;
}

pub fn create_translator_from_config(config: PhysicalTranslationStrategy)
                                     -> Box<dyn VirtualToPhysicalTranslator>
{
    match config {
        PhysicalTranslationStrategy::Selfmap(sm_config) => {
            Box::new(PagemapStrategy::new(sm_config))
        },
        PhysicalTranslationStrategy::Hypercall(hc_config) => {
            Box::new(KvmHypercallStrategy::new(hc_config))
        }
    }
}