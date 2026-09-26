// Lux Cargo Package Manager - Módulo de Integração
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct CargoToml {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

impl CargoToml {
    fn new(name: &str, version: &str, authors: &[&str]) -> Self {
        CargoToml {
            package: Package {
                name: name.to_string(),
                version: version.to_string(),
                authors: authors.iter().map(|&s| s.to_string()).collect(),
            },
            dependencies: HashMap::new(),
        }
    }

    fn add_dependency(&mut self, name: &str, version: &str) {
        self.dependencies.insert(name.to_string(), version.to_string());
    }

    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "[package]")?;
        writeln!(file, "name = \"{}\"", self.package.name)?;
        writeln!(file, "version = \"{}\"", self.package.version)?;
        for author in &self.package.authors {
            writeln!(file, "authors = [\"{}\"]", author)?;
        }
        writeln!(file)?;

        if !self.dependencies.is_empty() {
            writeln!(file, "[dependencies]")?;
            for (name, version) in &self.dependencies {
                writeln!(file, "{} = \"{}\"", name, version)?;
            }
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut cargo_toml = CargoToml::new("lux", "0.1.0", &["Your Name"]);
    cargo_toml.add_dependency("serde", "1.0");
    cargo_toml.save_to_file("Cargo.toml")?;

    Ok(())
}
