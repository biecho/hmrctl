use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{PhysicalAddress, V2PError, VirtualAddress};

const PAGE_SIZE: u64 = 4096;
const PAGEMAP_ENTRY_SIZE: u64 = 8;

pub struct PagemapStrategy {
    pagemap_path: String,
}

impl PagemapStrategy {
    pub fn new(pagemap_path: &str) -> Self {
        PagemapStrategy {
            pagemap_path: pagemap_path.to_string(),
        }
    }

    pub fn translate_to_physical(&self, virtual_address: VirtualAddress)
                                 -> Result<PhysicalAddress, V2PError>
    {
        let mut file = File::open(&self.pagemap_path)
            .map_err(V2PError::IoError)?;

        let index = virtual_address / PAGE_SIZE * PAGEMAP_ENTRY_SIZE;
        file.seek(SeekFrom::Start(index))
            .map_err(V2PError::IoError)?;

        let mut buffer = [0; 8];
        file.read_exact(&mut buffer)
            .map_err(V2PError::IoError)?;

        let page_frame_number = u64::from_ne_bytes(buffer) & 0x7FFFFFFFFFFFFF;
        let phys_addr = page_frame_number * PAGE_SIZE + (virtual_address % PAGE_SIZE);

        Ok(phys_addr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_valid_translation_with_mock() {
        // Create a mock pagemap file
        let mut tmp_file = NamedTempFile::new().expect("Failed to create tempfile.");

        // Write some mock data to the temporary file.
        tmp_file.write_all(&[0; 8 * PAGE_SIZE as usize]).expect("Failed to write to tempfile.");

        // Using our mock path to test the logic
        let mock_path = tmp_file.path().to_str().expect("Failed to convert path.");
        let strategy = PagemapStrategy::new(mock_path);
        let virtual_address = 0;  // Since we're mocking, we can use a simple address

        let physical_address = strategy.translate_to_physical(virtual_address).unwrap();

        assert_ne!(physical_address, 0);  // Simple check to ensure translation did not fail
    }
}
