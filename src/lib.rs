use std::fs;
use std::path::Path;
use std::error::Error;

/// Enum to represent supported configuration formats
#[derive(Debug)]
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

/// Load and parse a configuration file based on its format
pub fn load_config<P: AsRef<Path>>(path: P, format: ConfigFormat) -> Result<serde_json::Value, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    // Match the specified format and parse accordingly
    let config = match format {
        ConfigFormat::Json => serde_json::from_str(&content)?,
        ConfigFormat::Toml => {
            let toml_value: toml::Value = toml::from_str(&content)?;
            serde_json::to_value(toml_value)?
        },
        ConfigFormat::Yaml => serde_yaml::from_str(&content)?,
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_config() {
        let config = r#"{ "key": "value", "number": 42 }"#;
        fs::write("test_config.json", config).unwrap();
        let result = load_config("test_config.json", ConfigFormat::Json).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
        fs::remove_file("test_config.json").unwrap();
    }

    #[test]
    fn test_toml_config() {
        let config = r#"
key = "value"
number = 42
"#;
        fs::write("test_config.toml", config).unwrap();
        let result = load_config("test_config.toml", ConfigFormat::Toml).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
        fs::remove_file("test_config.toml").unwrap();
    }

    #[test]
    fn test_yaml_config() {
        let config = r#"
key: value
number: 42
"#;
        fs::write("test_config.yaml", config).unwrap();
        let result = load_config("test_config.yaml", ConfigFormat::Yaml).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
        fs::remove_file("test_config.yaml").unwrap();
    }
}
