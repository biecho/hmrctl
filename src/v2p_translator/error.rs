use std::fmt;

#[derive(Debug)]
pub enum V2PError {
    // Error reading from /proc/self/pagemap or other related I/O errors
    IoError(std::io::Error),

    // A virtual address is not mapped to a physical address
    UnmappedAddress,

    // Error related to the hypervisor, e.g., if using a virtualization-based strategy
    HypervisorError(String),

    // A generic error, for cases not covered above
    Other(String),
}

impl std::error::Error for V2PError {}

impl fmt::Display for V2PError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V2PError::IoError(err) => write!(f, "I/O error: {}", err),
            V2PError::UnmappedAddress => write!(f, "The virtual address is not mapped to a physical address"),
            V2PError::HypervisorError(err) => write!(f, "Hypervisor error: {}", err),
            V2PError::Other(err) => write!(f, "Other error: {}", err),
            // ... handle other error variants similarly
        }
    }
}

impl From<std::io::Error> for V2PError {
    fn from(err: std::io::Error) -> Self {
        V2PError::IoError(err)
    }
}
