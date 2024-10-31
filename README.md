# glider_config

`glider_config` is a minimal Rust library for loading configuration files in JSON, TOML, and YAML formats. This crate is designed to keep things lightweight and simple, making it easy to integrate basic configuration management into your Rust projects.

## Installation

Add `glider_config` to your `Cargo.toml`:

```toml
[dependencies]
glider_config = "0.1.0"
```
## Features

Supports JSON, TOML, and YAML configuration formats.
Loads configurations into a single, easy-to-use serde_json::Value format.
Minimal dependencies for quick and easy integration.

## Usage
Here's a basic example of how to use glider_config to load a configuration file:

```rust

use glider_config::{load_config, ConfigFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a JSON config file
    let config = load_config("path/to/config.json", ConfigFormat::Json)?;
    
    // Access configuration values
    if let Some(volume) = config.get("volume") {
        println!("Volume setting: {}", volume);
    }

    Ok(())
}
Supported Formats
JSON: Use ConfigFormat::Json to load JSON configuration files.
TOML: Use ConfigFormat::Toml to load TOML files.
YAML: Use ConfigFormat::Yaml for YAML configuration files.
```

## Example Code
Below is an example showing how to load and print values from each configuration type:

```rust

use glider_config::{load_config, ConfigFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a TOML configuration file
    let config = load_config("path/to/config.toml", ConfigFormat::Toml)?;
    println!("Loaded configuration: {:?}", config);

    // Access a specific value
    if let Some(value) = config.get("key") {
        println!("Value for 'key': {:?}", value);
    }
    
    Ok(())
}
```
## Running Tests
Run the test suite with:
cargo test

## License
This project is licensed under the MIT License.

## Author
Ben Santora (<bensatlantik@gmail.com>)