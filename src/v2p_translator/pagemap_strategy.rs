use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{PhysicalAddress, V2PError, VirtualAddress};

macro_rules! bit {
    ($x:expr) => {
        1u64 << $x
    };
}

macro_rules! mask {
    ($start:expr, $end:expr) => {
        ((1u64 << ($end - $start + 1)) - 1) << $start
    };
}

macro_rules! is_set {
    ($entry:expr, $flag:expr) => {
        $entry & $flag != 0
    };
}

macro_rules! extract_bits {
    ($entry:expr, $mask:expr) => {
        $entry & $mask
    };
}

/// The size of a memory page in bytes.
const PAGE_SIZE: u64 = 4096;

/// The size of an entry in the /proc/pid/pagemap file in bytes.
const PAGEMAP_ENTRY_SIZE: u64 = 8;

/// A mask to extract the page frame number (PFN) from a pagemap entry.
const PFN_MASK: u64 = mask!(0, 54);

/// A flag in the pagemap entry indicating if the page is present in memory.
const PAGE_PRESENT: u64 = bit!(63);

/// Represents a strategy for translating virtual addresses to physical
/// addresses using the Linux /proc/pid/pagemap interface.
pub struct PagemapStrategy {
    /// Path to the pagemap file for the target process.
    pagemap_path: String,
}

impl PagemapStrategy {
    /// Constructs a new `PagemapStrategy` for a given process pagemap path.
    ///
    /// # Arguments
    ///
    /// * `pagemap_path`: The path to the /proc/pid/pagemap file for the target process.
    pub fn new(pagemap_path: &str) -> Self {
        PagemapStrategy {
            pagemap_path: pagemap_path.to_string(),
        }
    }

    /// Translates a virtual address to its corresponding physical address.
    ///
    /// This function reads the pagemap entry for the virtual address and calculates
    /// the physical address based on the page frame number (PFN) and the offset
    /// within the page.
    ///
    /// # Arguments
    ///
    /// * `virtual_address`: The virtual address to be translated.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the translated physical address or an error.
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

        let entry = u64::from_ne_bytes(buffer);
        if !is_set!(entry, PAGE_PRESENT) {
            return Err(V2PError::UnmappedAddress);
        }

        let page_frame_number = extract_bits!(entry, PFN_MASK);
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
        // Mocked pagemap entry for a page that's present with a PFN of 12345.
        let pfn = 12345u64;
        let entry = (pfn & PFN_MASK) | PAGE_PRESENT;
        let mock_entry = entry.to_ne_bytes();

        // Create a mock pagemap file.
        let mut tmp_file = NamedTempFile::new().expect("Failed to create tempfile.");

        // Write the mocked entry to the temporary file.
        tmp_file.write_all(&mock_entry).expect("Failed to write to tempfile.");

        // Using our mock path to test the logic.
        let mock_path = tmp_file.path().to_str().expect("Failed to convert path.");
        let strategy = PagemapStrategy::new(mock_path);

        // Assuming the virtual address corresponding to the start of our tempfile.
        let virtual_address = 0;

        let physical_address = strategy.translate_to_physical(virtual_address).unwrap();

        assert_eq!(physical_address, pfn * PAGE_SIZE);
    }

}
