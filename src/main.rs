mod cli;

use std::{
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{bail, Result};
use clap::Parser;
use colored::*;
use hex::encode;
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::cli::{ChecksumType, Cli, Command};

fn check_valid_output_file_type(file_name: &Path) -> Result<()> {
    let error_msg = "Output file must be a .txt file";
    if let Some(extension) = file_name.extension() {
        if let Some(ext_str) = extension.to_str() {
            if ext_str != "txt" {
                bail!(error_msg);
            }
        } else {
            bail!(error_msg);
        }
    } else {
        bail!(error_msg);
    }

    Ok(())
}

fn print_error(message: &str) {
    eprintln!("{}", message.red());
}

fn process_checksum(
    file_path: &Path,
    output_file: &Option<PathBuf>,
    checksum_type: &ChecksumType,
    overwrite: bool,
    verbose: bool,
) -> Result<()> {
    if file_path.is_dir() {
        bail!(
            "{:?} is a direcotry and cannot be opened as a file",
            file_path
        );
    }

    let file_name = match file_path.file_name() {
        Some(f) => {
            if let Some(file_string) = f.to_str() {
                file_string
            } else {
                bail!("Error getting file name");
            }
        }
        None => {
            bail!("Error getting file name");
        }
    };

    let hash = match checksum_type {
        ChecksumType::Sha256 => {
            let bytes = fs::read(file_path);
            if let Ok(b) = bytes {
                let mut hasher = Sha256::new();
                hasher.update(b);
                encode(hasher.finalize())
            } else {
                bail!("Error opening file");
            }
        }
        ChecksumType::Sha1 => {
            let bytes = fs::read(file_path);
            if let Ok(b) = bytes {
                let mut hasher = Sha1::new();
                hasher.update(b);
                encode(hasher.finalize())
            } else {
                bail!("Error opening file");
            }
        }
        ChecksumType::Md5 => {
            let bytes = fs::read(file_path);
            if let Ok(b) = bytes {
                format!("{:?}", md5::compute(b))
            } else {
                bail!("Error opening file");
            }
        }
    };

    let checksum_output = format!("{checksum_type} checksum: {hash} - {file_name}");

    if let Some(o) = output_file {
        let output_path = o.parent();
        if let Some(path) = output_path {
            if !path.exists() && fs::create_dir_all(path).is_err() {
                bail!("Error creating directory");
            }
        }

        if !overwrite && o.exists() {
            let file_result = OpenOptions::new().append(true).open(o);

            if let Ok(mut file) = file_result {
                if let Err(e) = writeln!(file, "{}", &checksum_output) {
                    bail!("Couldn't write to file: {}", e);
                }
            } else {
                bail!("Error opening file");
            }
        } else {
            let file = File::create(o);

            if let Ok(mut f) = file {
                if let Err(e) = writeln!(f, "{}", &checksum_output) {
                    bail!("Error writing file: {}", e);
                }
            } else {
                bail!("Error writing file");
            }
        }
    }

    if output_file.is_none() || verbose {
        println!("{checksum_output}");
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            checksum_path,
            output_file,
            checksum_type,
            overwrite,
            verbose,
        } => {
            if !checksum_path.exists() {
                print_error(&format!("Path {:?} does not exist", checksum_path));
                exit(1);
            }

            if let Some(o) = &output_file {
                if let Err(e) = check_valid_output_file_type(o) {
                    print_error(&e.to_string());
                    exit(1);
                }
            }

            if checksum_path.is_file() {
                if let Err(e) = process_checksum(
                    &checksum_path,
                    &output_file,
                    &checksum_type,
                    overwrite,
                    verbose,
                ) {
                    print_error(&e.to_string());
                    exit(1);
                }
            } else if let Ok(dir) = fs::read_dir(&checksum_path) {
                for child_result in dir {
                    if let Ok(child) = child_result {
                        if child.path().is_file() {
                            if let Err(e) = process_checksum(
                                &child.path(),
                                &output_file,
                                &checksum_type,
                                overwrite,
                                verbose,
                            ) {
                                print_error(&e.to_string());
                                exit(1);
                            }
                        }
                    } else {
                        print_error("Error reading file");
                        exit(1);
                    }
                }
            } else {
                print_error(&format!(
                    "Error processing files in {:?} directory",
                    &checksum_path
                ));
                exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fake_file_path() -> PathBuf {
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let file_path = base.join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"This is a test").unwrap();

        file_path
    }

    fn fake_file_path_2() -> PathBuf {
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let file_path = base.join("test2.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"This is another test").unwrap();

        file_path
    }

    fn get_checksum(file: &Path, checksum_type: ChecksumType) -> String {
        match checksum_type {
            ChecksumType::Sha256 => {
                let bytes = fs::read(file).unwrap();
                let mut hasher = Sha256::new();
                hasher.update(bytes);
                encode(hasher.finalize())
            }
            ChecksumType::Sha1 => {
                let bytes = fs::read(file).unwrap();
                let mut hasher = Sha1::new();
                hasher.update(bytes);
                encode(hasher.finalize())
            }
            ChecksumType::Md5 => {
                let bytes = fs::read(file).unwrap();
                format!("{:?}", md5::compute(bytes))
            }
        }
    }

    #[test]
    fn check_valid_output_file_type_success() {
        assert!(check_valid_output_file_type(Path::new("test.txt")).is_ok());
    }

    #[test]
    fn check_invalid_output_file_type() {
        assert!(check_valid_output_file_type(Path::new("test.csv")).is_err());
    }

    #[test]
    fn process_checksum_directory_error() {
        let dir = tempdir().unwrap().path().to_path_buf();
        assert!(process_checksum(&dir, &None, &ChecksumType::Sha256, false, false).is_err());
    }

    #[test]
    fn process_checksum_bad_path() {
        let dir = Path::new("bad");
        assert!(process_checksum(dir, &None, &ChecksumType::Sha256, false, false).is_err());
    }

    #[test]
    fn generate_md5_file() {
        let test_file = fake_file_path();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file,
            &Some(output_file.clone()),
            &ChecksumType::Md5,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum = get_checksum(&test_file, ChecksumType::Md5);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum));
    }

    #[test]
    fn generate_md5_file_directory_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Md5,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Md5);
        let result_1 = fs::read_to_string(&output_file).unwrap();

        assert!(result_1.contains(&checksum_1));

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Md5,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Md5);
        let result_2 = fs::read_to_string(&output_file).unwrap();

        assert!(!result_2.contains(&checksum_1));
        assert!(result_2.contains(&checksum_2));
    }

    #[test]
    fn generate_md5_file_directory_no_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Md5,
            false,
            false,
        )
        .unwrap();

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Md5,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Md5);
        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Md5);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum_1));
        assert!(result.contains(&checksum_2));
    }

    #[test]
    fn generate_sha256_file() {
        let test_file = fake_file_path();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file,
            &Some(output_file.clone()),
            &ChecksumType::Sha256,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum = get_checksum(&test_file, ChecksumType::Sha256);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum));
    }

    #[test]
    fn generate_sha256_file_directory_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Sha256,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Sha256);
        let result_1 = fs::read_to_string(&output_file).unwrap();

        assert!(result_1.contains(&checksum_1));

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Sha256,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Sha256);
        let result_2 = fs::read_to_string(&output_file).unwrap();

        assert!(!result_2.contains(&checksum_1));
        assert!(result_2.contains(&checksum_2));
    }

    #[test]
    fn generate_sha256_file_directory_no_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Sha256,
            false,
            false,
        )
        .unwrap();

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Sha256,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Sha256);
        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Sha256);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum_1));
        assert!(result.contains(&checksum_2));
    }

    #[test]
    fn generate_sha1_file() {
        let test_file = fake_file_path();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file,
            &Some(output_file.clone()),
            &ChecksumType::Sha1,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum = get_checksum(&test_file, ChecksumType::Sha1);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum));
    }

    #[test]
    fn generate_sha1_file_directory_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Sha1,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Sha1);
        let result_1 = fs::read_to_string(&output_file).unwrap();

        assert!(result_1.contains(&checksum_1));

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Sha1,
            true,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Sha1);
        let result_2 = fs::read_to_string(&output_file).unwrap();

        assert!(!result_2.contains(&checksum_1));
        assert!(result_2.contains(&checksum_2));
    }

    #[test]
    fn generate_sha1_file_directory_no_overwrite() {
        let test_file_1 = fake_file_path();
        let test_file_2 = fake_file_path_2();
        let base = tempdir().unwrap().path().to_path_buf();
        fs::create_dir_all(&base).unwrap();
        let output_file = base.join("output.txt");

        process_checksum(
            &test_file_1,
            &Some(output_file.clone()),
            &ChecksumType::Sha1,
            false,
            false,
        )
        .unwrap();

        process_checksum(
            &test_file_2,
            &Some(output_file.clone()),
            &ChecksumType::Sha1,
            false,
            false,
        )
        .unwrap();

        assert!(output_file.exists());

        let checksum_1 = get_checksum(&test_file_1, ChecksumType::Sha1);
        let checksum_2 = get_checksum(&test_file_2, ChecksumType::Sha1);
        let result = fs::read_to_string(&output_file).unwrap();

        assert!(result.contains(&checksum_1));
        assert!(result.contains(&checksum_2));
    }
}
