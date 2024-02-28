use std::{fmt, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum ChecksumType {
    Md5,
    Sha1,
    Sha256,
}

impl fmt::Display for ChecksumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChecksumType::Md5 => write!(f, "md5"),
            ChecksumType::Sha1 => write!(f, "sha1"),
            ChecksumType::Sha256 => write!(f, "sha256"),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "Generate and validate checksums for files")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate checksums for files
    Generate {
        #[clap(help = "Path to the directory or file for which to generate checksums")]
        checksum_path: PathBuf,

        #[clap(
            short,
            long,
            help = "Path to the file to same the checksums. Must be a .txt file. If no path is provided the output will be printed to the screen and not saved"
        )]
        output_file: Option<PathBuf>,

        #[clap(
            short,
            long,
            default_value_t = ChecksumType::Sha256,
            help = "The type of checksum to generate"
        )]
        checksum_type: ChecksumType,

        #[clap(
            long,
            default_value_t = false,
            help = "Overwrite the output file rather than appending to it"
        )]
        overwrite: bool,

        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Provides more output while running"
        )]
        verbose: bool,
    },
}
