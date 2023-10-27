use crate::config::models::DramLayout;

/// Provides functionality to translate between physical and DRAM addresses
/// based on a specified DRAM layout configuration.
pub struct DramTranslator {
    layout: DramLayout,
}

type PhysAddr = u64;

#[derive(Debug, PartialEq)]
pub struct DramAddr {
    bank: u64,
    row: u64,
    col: u64,
}

impl DramTranslator {
    /// Creates a new DRAM translator based on a specific layout configuration.
    ///
    /// The `DramLayout` provides details about the memory's row, column, and bank
    /// addressing schemes, which are then used to map between physical and DRAM addresses.
    pub fn new(layout: DramLayout) -> Self {
        Self { layout }
    }


    /// Computes the DRAM row corresponding to a physical address.
    ///
    /// The function uses bitwise operations to isolate the row bits
    /// from the physical address and then shifts them to obtain the row value.
    fn get_dram_row(&self, physical_addr: PhysAddr) -> u64 {
        (physical_addr & self.layout.row_mask) >> self.layout.row_mask.trailing_zeros()
    }

    /// Computes the DRAM column corresponding to a physical address.
    ///
    /// Similar to `get_dram_row`, it uses bitwise operations to isolate
    /// the column bits from the physical address.
    fn get_dram_col(&self, physical_addr: PhysAddr) -> u64 {
        (physical_addr & self.layout.col_mask) >> self.layout.col_mask.trailing_zeros()
    }

    /// Translates a physical address to a DRAM address.
    ///
    /// The function calculates the bank, row, and column in the DRAM address
    /// corresponding to the given physical address. The bank address is determined by
    /// the parity of masked portions of the physical address as defined by
    /// `addressing_functions` in the layout.
    pub fn phys_to_dram(&self, physical_addr: PhysAddr) -> DramAddr {
        let bank = self.layout.addressing_functions.iter()
            .enumerate()
            .map(|(index, &mask)| (parity(physical_addr & mask) as u64) << index)
            .fold(0, |acc, bank_bit| acc | bank_bit);

        let row = self.get_dram_row(physical_addr);
        let col = self.get_dram_col(physical_addr);

        DramAddr { bank, row, col }
    }


    /// Translates a DRAM address back to its corresponding physical address.
    ///
    /// This function works in reverse compared to `phys_to_dram`. It reconstitutes
    /// the physical address from the DRAM address using the layout configuration.
    /// The tricky part is computing the physical bits that aren't explicitly part
    /// of the row or column — these bits are determined using the bank address and
    /// the `addressing_functions` from the layout.
    pub fn dram_to_phys(&self, dram_address: &DramAddr) -> PhysAddr {
        // Convert the row and col values of the DRAM address to the base part of the physical address.
        let base_physical_address =
            (dram_address.row << self.layout.row_mask.trailing_zeros())
                | (dram_address.col << self.layout.col_mask.trailing_zeros());

        // Iteratively process each addressing function to determine the final physical address.
        self.layout.addressing_functions.iter().enumerate()
            .fold(base_physical_address,
                  |current_physical, (function_index, &mask)| {

                // Mask the current physical address with the mapping function to isolate relevant bits.
                let relevant_bits = current_physical & mask;

                // Determine the expected parity for the current function using the bank information.
                let expected_parity = (dram_address.bank >> function_index) & 1;

                // Check if the parity of the relevant bits matches the expected parity.
                let has_correct_parity = parity(relevant_bits) == expected_parity as u8;

                if has_correct_parity {
                    current_physical
                } else {
                    // Identify and compute the specific bit in the address that needs to be toggled.
                    let bit_mask_excluding_row_col = !self.layout.col_mask & !self.layout.row_mask;
                    let bit_needing_flip = (mask & bit_mask_excluding_row_col)
                        .trailing_zeros();

                    // Return the adjusted address with the bit toggled.
                    current_physical ^ (1 << bit_needing_flip)
                }
            })
    }

}

/// Computes the parity of a number using its bit representation.
///
/// This function counts the number of set bits in `n` and returns 0 if
/// there's an even number of set bits, and 1 otherwise. Parity is useful
/// in various memory addressing schemes, especially for determining bank addresses.
pub fn parity(n: u64) -> u8 {
    (n.count_ones() % 2) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dram_to_phys_and_back() {
        let layout = DramLayout {
            row_mask: 0b_1111_0000_0000,
            col_mask: 0b_0000_1111_0000,
            addressing_functions: vec![0b_0000_0000_1001, 0b_0000_0000_0110],
        };
        let translator = DramTranslator::new(layout);

        let dram_address = DramAddr { bank: 2, row: 5, col: 3 };

        // Translate DRAM to Physical.
        let physical_address = translator.dram_to_phys(&dram_address);

        // Translate back from Physical to DRAM.
        let translated_dram = translator.phys_to_dram(physical_address);

        assert_eq!(dram_address, translated_dram);
    }

    #[test]
    fn test_parity() {
        assert_eq!(parity(0b_0101), 0);
        assert_eq!(parity(0b_0111), 1);
        assert_eq!(parity(0b_1000_0000_0000_0011), 1);
        assert_eq!(parity(0), 0);
    }

    #[test]
    fn test_get_dram_row_col() {
        let layout = DramLayout {
            row_mask: 0b_1111_0000_0000,
            col_mask: 0b_0000_1111_0000,
            addressing_functions: vec![0b_0000_0000_1001, 0b_0000_0000_0110],
        };
        let translator = DramTranslator::new(layout);

        let physical_addr = 0b_1010_1100_1011;

        assert_eq!(translator.get_dram_row(physical_addr), 0b_1010);
        assert_eq!(translator.get_dram_col(physical_addr), 0b_1100);
    }
}
