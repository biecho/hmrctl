// src/config/loader.rs

use std::fs;
use serde_yaml;
use super::models::Config;

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_valid_file() {
        let path = "src/config/test_files/valid_config.yml";
        let result = load_config(path);

        assert!(result.is_ok(), "Failed to load a valid config file");

        let config = result.unwrap();
        assert_eq!(config.memory.allocation.hugepages_mount, "/tmp/hugepages");
        assert_eq!(config.memory.allocation.size_bytes, 1073741824);
        // ... (additional assertions based on the structure and expected values)
    }

    #[test]
    fn test_load_config_invalid_file() {
        let path = "src/config/test_files/invalid_config.yml";
        let result = load_config(path);

        assert!(result.is_err(), "Managed to load an invalid config file");
    }

}
