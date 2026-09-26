use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Dependency = (String, String);

struct CargoPackage {
    name: String,
    version: String,
    dependencies: Vec<Dependency>,
}

impl CargoPackage {
    fn new(name: &str, version: &str) -> Self {
        CargoPackage {
            name: name.to_string(),
            version: version.to_string(),
            dependencies: Vec::new(),
        }
    }

    fn add_dependency(&mut self, dep_name: &str, dep_version: &str) {
        self.dependencies.push((dep_name.to_string(), dep_version.to_string()));
    }

    fn write_to_toml(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        writeln!(file, "[package]")?;
        writeln!(file, "name = \"{}\"", self.name)?;
        writeln!(file, "version = \"{}\"", self.version)?;
        writeln!(file, "edition = \"2018\"")?;
        writeln!(file)?;

        if !self.dependencies.is_empty() {
            writeln!(file, "[dependencies]")?;
            for (dep_name, dep_version) in &self.dependencies {
                writeln!(file, "{} = \"{}\"", dep_name, dep_version)?;
            }
        }

        Ok(())
    }

    fn read_from_toml(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut package = CargoPackage::new("", "");
        let mut in_dependencies = false;

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("[package]") {
                continue;
            } else if line.starts_with("[dependencies]") {
                in_dependencies = true;
                continue;
            } else if line.trim().is_empty() {
                continue;
            }

            if !in_dependencies {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                match parts[0] {
                    "name" => package.name = parts[1].trim_matches('"').to_string(),
                    "version" => package.version = parts[1].trim_matches('"').to_string(),
                    _ => {}
                }
            } else {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    package.add_dependency(parts[0], parts[1]);
                }
            }
        }

        Ok(package)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut package = CargoPackage::new("lux", "0.1.0");
    package.add_dependency("serde", "1.0");
    package.add_dependency("tokio", "1.0");

    let path = Path::new("Cargo.toml");
    package.write_to_toml(&path)?;

    println!("Cargo.toml created successfully.");

    let read_package = CargoPackage::read_from_toml(&path)?;
    println!("Read from Cargo.toml: {:?}", read_package);

    Ok(())
}
