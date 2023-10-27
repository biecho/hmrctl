use super::{PhysicalAddress, V2PError, VirtualAddress};

pub struct KvmHypercallStrategy<F = fn(VirtualAddress) -> Result<PhysicalAddress, V2PError>>
    where
        F: Fn(VirtualAddress) -> Result<PhysicalAddress, V2PError>,
{
    translate_fn: F,
}

struct HypercallArgs {
    pub virt_addr: VirtualAddress,
    pub phys_addr: PhysicalAddress,
}

impl KvmHypercallStrategy<fn(VirtualAddress) -> Result<PhysicalAddress, V2PError>> {
    pub fn new() -> Self {
        KvmHypercallStrategy { translate_fn: real_hypercall }
    }
}

impl<F> KvmHypercallStrategy<F>
    where
        F: Fn(VirtualAddress) -> Result<PhysicalAddress, V2PError>,
{
    pub fn with_translate_fn(translate_fn: F) -> Self {
        KvmHypercallStrategy { translate_fn }
    }

    pub fn translate_to_physical(&self, virt_addr: VirtualAddress)
                                 -> Result<PhysicalAddress, V2PError> {
        (self.translate_fn)(virt_addr)
    }
}

#[cfg(target_arch = "x86_64")]
fn real_hypercall(virt_addr: VirtualAddress) -> Result<PhysicalAddress, V2PError> {
    let mut args = HypercallArgs {
        virt_addr,
        phys_addr: 0,  // Initial value, will be populated by the hypervisor
    };

    unsafe {
        asm!(
        "vmcall",
        inout("rax") 14 => _, // rax register will contain the hypercall number
        inout("rbx") &args => _, // rbx register might contain the pointer to our arguments
        // Additional setup might be required depending on the calling convention
        );
    }

    if args.phys_addr == 0 {
        Err(V2PError::Other("Hypervisor translation failed".to_string()))
    } else {
        Ok(args.phys_addr)
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn real_hypercall(_virt_addr: VirtualAddress) -> Result<PhysicalAddress, V2PError> {
    // Return an error for non-x86_64 platforms
    Err(V2PError::Other("Not supported on non-x86_64 platforms".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mocked_translation() {
        let strategy = KvmHypercallStrategy::with_translate_fn(|virt_addr| {
            Ok(virt_addr + 10)  // mock
        });

        let virt_addr = 5;
        let phys_addr = strategy.translate_to_physical(virt_addr).unwrap();

        assert_eq!(phys_addr, 15);
    }
}
