use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{stderr, Error, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};



const HASH_CAT: &str = "C:\\Users\\lawor\\Downloads\\hashcat-7.1.1\\hashcat-7.1.1\\hashcat.exe";
const HASH_FILE: &str = "C:\\Users\\lawor\\Documents\\Lab7\\HashValues.txt";
const WORDLIST: &str = "C:\\Users\\lawor\\Documents\\Lab7\\rockyou.txt";

fn modes() -> HashMap<&'static str, u16> {
    HashMap::from([
        ("MD4", 500),
        ("MD5", 0),
        ("SHA1", 100),
        ("SHA2-224", 1300),
        ("SHA2-256", 1400),
        ("SHA2-384", 10800),
        ("SHA2-512", 1700),
        ("SHA3-224", 17300),
        ("SHA3-256", 17400),
        ("SHA3-384", 17500),
        ("SHA3-512", 17600),
        ("RIPEMD-160", 6000),
        ("BLAKE2b-512", 600),
        ("GOST-R-2012-256", 11700),
        ("GOST-R-2012-512", 11800),
        ("GOST-R-94", 6900),
        ("GPG-AES", 17010),
        ("Half-MD5", 5100),
        ("Keccak-224", 17700),
        ("Keccak-256", 17800),
        ("Keccak-384", 17900),
        ("Keccak-512", 18000),
        ("Whirlpool", 6100),
        ("SipHash", 10100),
    ])
}

fn executor() {
    let modes_map: HashMap<&str, u16> = modes();

    let hashcat_dir = Path::new(HASH_CAT)
        .parent()
        .unwrap_or(Path::new("."));

    for (name, mode) in modes_map.iter() {
        println!("Trying {} (mode {})...", name, mode);

        let result: Result<Output, std::io::Error> = Command::new(HASH_CAT)
            .current_dir(hashcat_dir)
            .args(&["-m", &mode.to_string(), "-a", "0", HASH_FILE, WORDLIST])
            .output();

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if output.status.success() {
                    println!("Mode {} ({}) finished with exit code 0.", mode, name);

                    if !stdout.trim().is_empty() {
                        println!("stdout:\n{}", stdout);
                    }

                    if !stderr.trim().is_empty() {
                        println!("stderr:\n{}", stderr);
                    }

                    match Command::new(HASH_CAT)
                        .current_dir(hashcat_dir)
                        .args(&["-m", &mode.to_string(), "--show", HASH_FILE])
                        .output()
                    {
                        Ok(show_out) => {
                            let show_stdout = String::from_utf8_lossy(&show_out.stdout);
                            let show_stderr = String::from_utf8_lossy(&show_out.stderr);

                            if !show_stdout.trim().is_empty() {
                                println!(
                                    "Recovered results for {} (mode {}):\n{}",
                                    name, mode, show_stdout
                                );
                            } else if !show_stderr.trim().is_empty() {
                                eprintln!(
                                    "hashcat --show stderr for mode {}: {}",
                                    mode, show_stderr
                                );
                            } else {
                                println!("No results from --show for {} (mode {}).", name, mode);
                            }
                        }
                        Err(e) => eprintln!("Failed to run hashcat --show for mode {}: {}", mode, e),
                    }
                } else {
                    eprintln!(
                        "Mode {} ({}) failed with exit code: {:?}.",
                        mode,
                        name,
                        output.status.code()
                    );

                    if !stdout.trim().is_empty() {
                        eprintln!("stdout:\n{}", stdout);
                    }

                    if !stderr.trim().is_empty() {
                        eprintln!("stderr:\n{}", stderr);
                    }

                    if stdout.trim().is_empty() && stderr.trim().is_empty() {
                        eprintln!(
                            "No stdout/stderr produced. Exit code: {:?}",
                            output.status.code()
                        );
                    }

                    println!("Retrying mode {} with --force for debugging...", mode);

                    if let Ok(force_out) = Command::new(HASH_CAT)
                        .current_dir(hashcat_dir)
                        .args(&["--force", "-m", &mode.to_string(), "-a", "0", HASH_FILE, WORDLIST])
                        .output()
                    {
                        let f_stdout = String::from_utf8_lossy(&force_out.stdout);
                        let f_stderr: Cow<'_, str> = String::from_utf8_lossy(&force_out.stderr);

                        eprintln!("--force stdout:\n{}", f_stdout);
                        eprintln!("--force stderr:\n{}", f_stderr);
                        eprintln!("--force exit code: {:?}", force_out.status.code());
                    }
                }
            }

            Err(e) => {
                eprintln!(
                    "Failed to execute hashcat for mode {} ({}): {}",
                    mode, name, e
                );
            }
        }
    }
}

fn file_loader() {
    let path: &Path = Path::new("C:\\Users\\lawor\\Documents\\Lab7");

    if !path.is_dir() {
        eprintln!("Provided path is not a directory: {}", path.display());
        return;
    }

    let mut has_hash_values: bool = false;
    let mut has_rock_you: bool = false;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e: Result<fs::DirEntry, std::io::Error>| e.ok()) {
            let entry_path: PathBuf = entry.path();

            if entry_path.is_file() {
                if let Some(file_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                    let lower_name = file_name.to_lowercase();

                    if lower_name.contains("hashvalues") {
                        has_hash_values = true;
                    }

                    if lower_name.contains("rockyou") {
                        has_rock_you = true;
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", path.display());
    }

    if !has_hash_values {
        eprintln!("No hash values file found in the directory.");
        return;
    }

    if !has_rock_you {
        eprintln!("No rockyou file found in the directory.");
        return;
    }

    executor();
}

fn main() {
    file_loader();
}
