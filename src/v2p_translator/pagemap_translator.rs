use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::config::models::SelfmapConfig;
use crate::v2p_translator::translator::VirtualToPhysicalTranslator;

use super::V2PError;

/// Represents a strategy for translating virtual addresses to physical
/// addresses using the Linux /proc/pid/pagemap interface.
pub struct PagemapStrategy {
    /// Configuration for selfmap translation strategy.
    config: SelfmapConfig,
}

impl PagemapStrategy {
    /// Constructs a new `PagemapStrategy` from the configuration.
    ///
    /// # Arguments
    ///
    /// * `config`: The selfmap configuration details.
    pub fn new(config: SelfmapConfig) -> Self {
        PagemapStrategy { config }
    }
}

impl VirtualToPhysicalTranslator for PagemapStrategy {
    fn translate_to_physical(&self, virtual_address: u64) -> Result<u64, V2PError> {
        let mut file = File::open(&self.config.pagemap_path)
            .map_err(V2PError::IoError)?;

        let index = virtual_address / self.config.page_size * self.config.pagemap_entry_size;
        file.seek(SeekFrom::Start(index))
            .map_err(V2PError::IoError)?;

        let mut buffer = [0; 8];
        file.read_exact(&mut buffer)
            .map_err(V2PError::IoError)?;

        let entry = u64::from_ne_bytes(buffer);
        if entry & self.config.page_present_mask == 0 {
            return Err(V2PError::UnmappedAddress);
        }

        let page_frame_number = entry & self.config.pfn_mask;
        let phys_addr = page_frame_number * self.config.page_size + (virtual_address % self.config.page_size);

        Ok(phys_addr)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_valid_translation_with_mock() {
        // Mocked pagemap entry for a page that's present with a PFN of 12345.
        let pfn = 12345u64;
        let entry = (pfn << 0) | (1u64 << 63); // Assuming the PFN_MASK starts at bit 0 and PAGE_PRESENT is at bit 63.
        let mock_entry = entry.to_ne_bytes();

        // Create a mock pagemap file.
        let mut tmp_file = NamedTempFile::new().expect("Failed to create tempfile.");

        // Write the mocked entry to the temporary file.
        tmp_file.write_all(&mock_entry).expect("Failed to write to tempfile.");

        // Using our mock path to test the logic.
        let mock_path = tmp_file.path().to_str().expect("Failed to convert path.");

        let config = SelfmapConfig {
            pagemap_path: mock_path.to_string(),
            page_size: 4096,
            pagemap_entry_size: 8,
            pfn_mask: (1u64 << 55) - 1, // Mask from bit 0 to 54 inclusive.
            page_present_mask: 1u64 << 63,
        };

        let strategy = PagemapStrategy::new(config);

        // Assuming the virtual address corresponding to the start of our tempfile.
        let virtual_address = 0;

        let physical_address = strategy.translate_to_physical(virtual_address).unwrap();

        assert_eq!(physical_address, pfn * 4096);
    }
}

