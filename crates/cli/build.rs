use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use which::which;

#[derive(Deserialize)]
struct Dependency {
    pub name: String,
    pub version: String,
}

fn main() {
    let package_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_path = package_path.parent().unwrap().parent().unwrap();
    let book_path = workspace_path.join("manual");

    for entry in book_path.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.file_name().is_some_and(|s| s == "site") {
            continue;
        }
        println!(
            "cargo::rerun-if-changed={}",
            book_path.join("src").display()
        );
    }

    if env::var("CARGO_FEATURE_MANUAL").is_ok() {
        let dependencies: Vec<Dependency> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../manual/dependencies.ron"
        )))
        .unwrap();

        for dep in dependencies {
            let installed = which(&dep.name).is_ok();
            let matching_version_installed = installed
                && match Command::new(&dep.name).arg("--version").output() {
                    Ok(output) => String::from_utf8_lossy(&output.stdout).contains(&dep.version),
                    Err(error) => {
                        println!("cargo::warning=Failed to invoke {}: {}", &dep.name, error);
                        false
                    }
                };

            if matching_version_installed {
                println!(
                    "cargo::warning={} ({}) already installed",
                    &dep.name, &dep.version
                );
                continue;
            }

            let mut child = Command::new("cargo")
                .arg("install")
                .arg("--force")
                .arg("--version")
                .arg(&dep.version)
                .arg(&dep.name)
                .spawn()
                .unwrap_or_else(|e| panic!("Failed to install {}: {}", dep.name, e));

            let status = child.wait().unwrap();
            if !status.success() {
                panic!("Installation failed with status {}", status);
            }
            // NOTE: messages are only printed when script is done thus "Installed" instead of
            //       "Installing"
            println!("cargo::warning=Installed {} ({})", &dep.name, &dep.version);
        }
    }

    fs::create_dir_all(workspace_path.join("target").join("manual"))
        .expect("Failed to create manual directory in target directory");

    let mut child = Command::new("mdbook")
        .arg("build")
        .arg(book_path)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to build book: {}", e));

    let status = child.wait().unwrap();
    if !status.success() {
        panic!("Failed to build book with status: {}", status);
    }
}
